use apigatewaymanagement::primitives::Blob;
use aws_sdk_apigatewaymanagement as apigatewaymanagement;
use aws_sdk_dynamodb::types::AttributeValue;
use core::{get_environment, EnvironmentVariables};
use lambda_http::{
    aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest},
    Body,
};
use lambda_runtime::{run, service_fn, Error as LambdaError, LambdaEvent};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();

    let environment_variables = get_environment().await;
    let dynamo_client = aws_sdk_dynamodb::Client::new(&environment_variables.aws_config);
    let apigateway_client = apigatewaymanagement::Client::from_conf(
        apigatewaymanagement::config::Builder::from(&environment_variables.aws_config)
            .endpoint_url(&environment_variables.gateway_management_url)
            .build(),
    );

    let environment_variables = &environment_variables;
    let dynamo_client = &dynamo_client;
    let apigateway_client = &apigateway_client;

    run(service_fn(move |e| async move {
        handler(e, environment_variables, dynamo_client, apigateway_client).await
    }))
    .await?;

    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    environment_variables: &EnvironmentVariables,
    dynamo_client: &aws_sdk_dynamodb::Client,
    apigateway_client: &aws_sdk_apigatewaymanagement::Client,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
    tracing::debug!("connect.handler: {:?}", event);

    let connection_id = match event.payload.request_context.connection_id {
        Some(connection_id) => connection_id,
        None => {
            return Ok(ApiGatewayProxyResponse {
                status_code: 400,
                headers: lambda_http::http::HeaderMap::new(),
                multi_value_headers: lambda_http::http::HeaderMap::new(),
                body: Some(Body::Text(
                    json!({
                        "message": "No connection id provided"
                    })
                    .to_string(),
                )),
                is_base64_encoded: false,
            })
        }
    };

    let put_item_request = dynamo_client
        .put_item()
        .table_name(environment_variables.connected_clients_table_name.clone())
        .item(
            &environment_variables.connected_clients_table_partition_key,
            AttributeValue::S(connection_id.to_string()),
        );

    tracing::debug!("dynamo.put_item: {:?}", put_item_request);

    put_item_request.send().await?;

    let scan_items_request = dynamo_client
        .scan()
        .table_name(environment_variables.connected_clients_table_name.clone())
        .limit(10);

    tracing::debug!("dynamo.scan: {:?}", scan_items_request);

    if let Some(items) = scan_items_request.send().await?.items {
        let connection_id = connection_id.as_str();
        let futures: Vec<_> = items
            .iter()
            .cloned()
            .map(|x| async move {
                let conn_id = x
                    .get(&environment_variables.connected_clients_table_partition_key)
                    .unwrap()
                    .as_s()
                    .unwrap();

                tracing::debug!("sending message to conn_id: {:?}", conn_id);

                apigateway_client
                    .post_to_connection()
                    .connection_id(conn_id)
                    .data(Blob::new(
                        json!({ "message": format!("User {} has entered the chat.", connection_id) })
                            .to_string(),
                    ))
                    .send()
                    .await
            })
            .collect();

        let send_results = futures::future::try_join_all(futures).await;
        tracing::debug!("send_results: {:?}", send_results);
    }

    let mut headers = lambda_http::http::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: lambda_http::http::HeaderMap::new(),
        body: None,
        is_base64_encoded: false,
    };

    Ok(resp)
}
