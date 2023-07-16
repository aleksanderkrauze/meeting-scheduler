use anyhow::Result;
use dotenvy::dotenv;

use backend::{run_server, Config};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Loading .env file");
    let _ = dotenv();

    let config = Config::from_env()?;

    run_server(config).await
}
