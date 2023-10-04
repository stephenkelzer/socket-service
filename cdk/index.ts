#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
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

    const connectLambda = new lambda.Function(this, 'ConnectLambda', {
      description: "SocketService Connect Lambda",
      code: lambda.AssetCode.fromAsset("./../target/lambda/connect/bootstrap.zip", { deployTime: true }),
      handler: "does_not_matter_for_rust_lambdas",
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      logRetention: RetentionDays.ONE_WEEK
    });

    const disconnectLambda = new lambda.Function(this, 'DisconnectLambda', {
      description: "SocketService Disconnect Lambda",
      code: lambda.AssetCode.fromAsset("./../target/lambda/disconnect/bootstrap.zip", { deployTime: true }),
      handler: "does_not_matter_for_rust_lambdas",
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      logRetention: RetentionDays.ONE_WEEK
    });

    const defaultLambda = new lambda.Function(this, 'DefaultLambda', {
      description: "SocketService Default Lambda",
      code: lambda.AssetCode.fromAsset("./../target/lambda/default/bootstrap.zip", { deployTime: true }),
      handler: "does_not_matter_for_rust_lambdas",
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      logRetention: RetentionDays.ONE_WEEK
    });

    const apiGateway = new cdkApiGateway.WebSocketApi(this, `${props.environment}-WebSocketGateway`, {
      description: "Websocket Gateway that proxies requests to the Rust Lambda functions",
      connectRouteOptions: {
          integration: new WebSocketLambdaIntegration('connect-integration', connectLambda)
      },
      disconnectRouteOptions: {
          integration: new WebSocketLambdaIntegration('disconnect-integration', disconnectLambda)
      },
      defaultRouteOptions: {
          integration: new WebSocketLambdaIntegration('default-integration', defaultLambda),
          returnResponse: true
      },
    });

    apiGateway.grantManageConnections(connectLambda);
    apiGateway.grantManageConnections(disconnectLambda);
    apiGateway.grantManageConnections(defaultLambda);

    apiGateway.addRoute('test', {
      integration: new WebSocketLambdaIntegration('default-integration', defaultLambda),
      returnResponse: true
    });

    new cdk.CfnOutput(this, 'GatewayUrl', { value: apiGateway.apiEndpoint ?? "unknown" });
  }
}

const environment: ENVIRONMENT = process.env.ENVIRONMENT as ENVIRONMENT || 'local';
const app = new cdk.App();
new SocketStack(app, `${environment}-SocketStack`, { environment });// set environment with env vars