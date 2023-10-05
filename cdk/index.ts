#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import * as cdkDynamoDB from 'aws-cdk-lib/aws-dynamodb';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as cdkApiGateway from '@aws-cdk/aws-apigatewayv2-alpha';
import { WebSocketLambdaIntegration } from '@aws-cdk/aws-apigatewayv2-integrations-alpha';
import { Construct } from 'constructs';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';

type ENVIRONMENT = 'test' | 'staging' | 'prod';

interface SocketStackProps extends cdk.StackProps {
    environment: ENVIRONMENT
}

class SocketStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: SocketStackProps) {
    super(scope, id, props);

    const db = new cdkDynamoDB.Table(this, 'SocketTable', {
      tableName: 'connected_clients',
      partitionKey: {
        name: 'connection_id',
        type: cdkDynamoDB.AttributeType.STRING
      },
      billingMode: cdkDynamoDB.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    const baseLambdaProps: cdk.aws_lambda.FunctionProps = {
      code: null as unknown as lambda.Code,
      handler: "does_not_matter_for_rust_lambdas",
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      logRetention: RetentionDays.ONE_WEEK,
      environment: {
        CONNECTED_CLIENTS_TABLE_NAME: db.tableName,
        CONNECTED_CLIENTS_TABLE_PARTITION_KEY: db.schema().partitionKey.name
      }
    }

    const connectLambda = new lambda.Function(this, 'ConnectLambda', {
      ...baseLambdaProps,
      description: "SocketService Connect Lambda",
      code: lambda.AssetCode.fromAsset("./../target/lambda/connect/bootstrap.zip", { deployTime: true }),
    });
    db.grantReadWriteData(connectLambda);

    const disconnectLambda = new lambda.Function(this, 'DisconnectLambda', {
      ...baseLambdaProps,
      description: "SocketService Disconnect Lambda",
      code: lambda.AssetCode.fromAsset("./../target/lambda/disconnect/bootstrap.zip", { deployTime: true }),
    });
    db.grantReadWriteData(disconnectLambda);

    const defaultLambda = new lambda.Function(this, 'DefaultLambda', {
      ...baseLambdaProps,
      description: "SocketService Default Lambda",
      code: lambda.AssetCode.fromAsset("./../target/lambda/default/bootstrap.zip", { deployTime: true }),
    });
    db.grantReadData(defaultLambda);

    const apiGateway = new cdkApiGateway.WebSocketApi(this, `${props.environment}-WebSocketGateway`, {
      description: "Websocket Gateway that proxies requests to the Rust Lambda functions",
      connectRouteOptions: {
          integration: new WebSocketLambdaIntegration('$connect', connectLambda),
          returnResponse: true
      },
      disconnectRouteOptions: {
          integration: new WebSocketLambdaIntegration('$disconnect', disconnectLambda),
          returnResponse: true
      },
      defaultRouteOptions: {
          integration: new WebSocketLambdaIntegration('$default', defaultLambda),
          returnResponse: true
      },
    });

    const apiGatewayStage = new cdkApiGateway.WebSocketStage(this, `${props.environment}-WebSocketStage`, {
      stageName: "abc",
      webSocketApi: apiGateway,
      autoDeploy: true
    });

    apiGateway.grantManageConnections(connectLambda);
    apiGateway.grantManageConnections(disconnectLambda);
    apiGateway.grantManageConnections(defaultLambda);

    new cdk.CfnOutput(this, 'GatewayUrl', { value: apiGatewayStage.url ?? "unknown" });
  }
}

const environment: ENVIRONMENT = process.env.ENVIRONMENT as ENVIRONMENT || 'local';
const app = new cdk.App();
new SocketStack(app, `${environment}-SocketStack`, { environment });// set environment with env vars