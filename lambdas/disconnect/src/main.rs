use lambda_http::aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest;
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

async fn handler(event: LambdaEvent<ApiGatewayWebsocketProxyRequest>) -> Result<(), LambdaError> {
    println!("disconnected.p: {:?}", event);
    tracing::trace!("disconnected.t: {:?}", event);
    tracing::debug!("disconnected.d: {:?}", event);
    tracing::info!("disconnected.i: {:?}", event);
    Ok(())
}
