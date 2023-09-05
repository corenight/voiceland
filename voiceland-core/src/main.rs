use std::{env, path::Path};

use anyhow::Result;
use config::config;

mod config;
mod structs;

#[tokio::main]
async fn main() -> Result<()> {
    let path =
        env::var("VOICELAND_CONFIG_PATH").unwrap_or("/etc/voiceland/core/config.yaml".into());

    let config = config(Path::new(&path)).await?;

    Ok(())
}
