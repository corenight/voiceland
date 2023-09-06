use std::{env, path::Path, process::exit};

use anyhow::Result;
use config::config;
use voiceland_common::logs;

mod config;
mod structs;

#[tokio::main]
async fn main() {
    match run().await {
        Err(err) => {
            logs::error(err.to_string());
            exit(1)
        }
        Ok(_) => (),
    };
}

async fn run() -> Result<()> {
    let path =
        env::var("VOICELAND_CONFIG_PATH").unwrap_or("/etc/voiceland/core/config.yaml".into());

    let config = config(Path::new(&path)).await?;

    Ok(())
}
