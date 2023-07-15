mod config;

use anyhow::Result;
use dotenvy::dotenv;

use config::Config;

fn main() -> Result<()> {
    let _ = dotenv();

    let config = Config::from_env()?;

    Ok(())
}
