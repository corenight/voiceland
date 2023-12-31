use std::path::Path;

use anyhow::{bail, Result};
use tokio::fs;
use voiceland_common::logs;

use crate::structs;

pub mod default;

/// Reads the configuration file to set several parameters of server.
///
/// By default reads into `/etc/voiceland/core/config.yaml`. This can be overridden with a custom path.
pub async fn config(dir: &Path) -> Result<structs::config::Configuration> {
    if !dir.exists() {
        bail!(
            "Config file path doesn't exists.\nNote: check if `/etc/voiceland/core/config.yaml` exists."
        )
    }

    // Get config
    let config = fs::read(dir).await?;
    let config: structs::config::Configuration = serde_yaml::from_slice(&config)?;

    // Check config
    if (config.tls_cert.cert.is_none() || config.tls_cert.key.is_none())
        && config.tls_cert.server_name.is_none()
    {
        bail!("At least TLS cert/key or server_name is required.")
    }

    // Warns
    if config.tls_cert.server_name.is_some()
        && (config.tls_cert.cert.is_none() || config.tls_cert.key.is_none())
    {
        logs::warn("You're running the server with a self-signed certificate.");
        logs::info(
            "Note: consider using a certificate. If you're sure what you're doing, ignore this.",
        );
    }

    Ok(config)
}
