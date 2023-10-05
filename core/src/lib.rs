use std::env;

use aws_config::SdkConfig;
use dotenvy::dotenv;

pub struct EnvironmentVariables {
    pub aws_config: SdkConfig,
    pub connected_clients_table_name: String,
    pub connected_clients_table_partition_key: String,
    pub gateway_management_url: String,
}

pub async fn get_environment() -> EnvironmentVariables {
    dotenv().ok();

    let aws_config = aws_config::load_from_env().await;

    return EnvironmentVariables {
        aws_config,
        connected_clients_table_name: env::var("CONNECTED_CLIENTS_TABLE_NAME")
            .expect("CONNECTED_CLIENTS_TABLE_NAME is a required environment variable"),
        connected_clients_table_partition_key: env::var("CONNECTED_CLIENTS_TABLE_PARTITION_KEY")
            .expect("CONNECTED_CLIENTS_TABLE_PARTITION_KEY is a required environment variable"),
        gateway_management_url: env::var("GATEWAY_MANAGEMENT_URL")
            .expect("GATEWAY_MANAGEMENT_URL is a required environment variable"),
    };
}

// pub fn init_tracing() {
//     tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::INFO)
//         .with_target(false)
//         .without_time()
//         .init();
// }
