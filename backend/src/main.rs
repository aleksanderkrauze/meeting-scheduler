use anyhow::Result;
use dotenvy::dotenv;

use backend::{run_server, Config};
use tracing_subscriber::{
    filter::{self, FilterExt},
    layer::{Layer, SubscriberExt},
    registry,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<()> {
    let stderr_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stderr)
        .with_filter(
            filter::LevelFilter::DEBUG.and(filter::filter_fn(|metadata| {
                metadata.target().starts_with("backend")
            })),
        );
    registry().with(stderr_layer).init();

    tracing::info!("Loading .env file");
    let _ = dotenv();

    let config = Config::from_env()?;

    run_server(config).await
}
