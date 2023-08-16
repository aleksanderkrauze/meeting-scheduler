use anyhow::{Context, Result};
use dotenvy::dotenv;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_subscriber::{
    filter,
    layer::{Layer, SubscriberExt},
    registry,
    util::SubscriberInitExt,
};

use backend::{run_server, Config};

#[tokio::main]
async fn main() -> Result<()> {
    let stderr_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stderr)
        .with_filter(filter::filter_fn(|metadata| {
            metadata.target().starts_with("backend")
        }));
    registry().with(stderr_layer).init();

    tracing::info!("Loading .env file");
    let _ = dotenv();

    let config = Config::from_env()?;

    let cancellation_token = CancellationToken::new();
    signal_handlers(cancellation_token.clone()).await?;

    run_server(config, cancellation_token).await
}

async fn signal_handlers(cancellation_token: CancellationToken) -> Result<()> {
    let mut sigterm =
        signal(SignalKind::terminate()).context("failed to register SIGTERM handler")?;
    let mut sigint =
        signal(SignalKind::interrupt()).context("failed to register SIGINT handler")?;

    tokio::spawn(async move {
        tokio::select! {
            sig = sigterm.recv() => {
                match sig {
                    Some(()) => info!("Received SIGTERM signal"),
                    None => error!("Received None from SIGTERM signal handler")
                }
                info!("Calling cancel on cancelation token");
                cancellation_token.cancel();
            }
            sig = sigint.recv() => {
                match sig {
                    Some(()) => info!("Received SIGINT signal"),
                    None => error!("Received None from SIGINT signal handler")
                }
                info!("Calling cancel on cancelation token");
                cancellation_token.cancel();
            }
        }
    });

    Ok(())
}
