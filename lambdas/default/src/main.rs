use lambda_http::aws_lambda_events::apigw::{
    ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest,
};
use lambda_runtime::{run, service_fn, Error as LambdaError, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    Logger::init();

    run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
    tracing::trace!("default.handler: {:?}", event);

    // Send the message to all connected clients
    // remove clients if the send fails?

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
