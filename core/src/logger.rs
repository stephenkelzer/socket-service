pub struct Logger {}

impl Logger {
    pub fn init() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .without_time()
            .init();
    }
}
