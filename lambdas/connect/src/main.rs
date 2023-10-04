use lambda_http::aws_lambda_events::apigw::{
    ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest,
};
use lambda_runtime::{run, service_fn, Error as LambdaError, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
    println!("connected.p: {:?}", event);
    tracing::trace!("connected.t: {:?}", event);
    tracing::debug!("connected.d: {:?}", event);
    tracing::info!("connected.i: {:?}", event);
    // Ok(json!({ "statusCode": 200 }))

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
