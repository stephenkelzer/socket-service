use std::env;

use aws_config::SdkConfig;
use dotenvy::dotenv;

pub struct Variables {
    pub aws_config: SdkConfig,
    pub connected_clients_table_name: String,
    pub connected_clients_table_partition_key: String,
    pub gateway_management_url: String,
}

impl Variables {
    pub async fn init() -> Self {
        dotenv().ok();

        let aws_config = aws_config::load_from_env().await;

        Self {
            aws_config,
            connected_clients_table_name: env::var("CONNECTED_CLIENTS_TABLE_NAME")
                .expect("CONNECTED_CLIENTS_TABLE_NAME is a required environment variable"),
            connected_clients_table_partition_key: env::var(
                "CONNECTED_CLIENTS_TABLE_PARTITION_KEY",
            )
            .expect("CONNECTED_CLIENTS_TABLE_PARTITION_KEY is a required environment variable"),
            gateway_management_url: env::var("GATEWAY_MANAGEMENT_URL")
                .expect("GATEWAY_MANAGEMENT_URL is a required environment variable"),
        }
    }
}
