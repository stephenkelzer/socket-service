use lambda_http::aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::{run, service_fn, Error as LambdaError, LambdaEvent};
use serde_json::json;

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
) -> Result<serde_json::Value, LambdaError> {
    println!("default.p: {:?}", event);
    tracing::trace!("default.t: {:?}", event);
    tracing::debug!("default.d: {:?}", event);
    tracing::info!("default.i: {:?}", event);

    Ok(json!({ "statusCode": 200 }))
}
