use std::{env, path::Path, process::exit, sync::Arc, time::Duration};

use anyhow::{bail, Result};
use quinn::Endpoint;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use tokio::{fs, sync::broadcast};

use config::{config, default};
use voiceland_common::logs;

mod config;
mod consts;
mod handler;
mod structs;
mod utils;

#[tokio::main]
async fn main() {
    // AGPL-3.0 notice
    println!("{}", consts::LICENSE_NOTICE);

    match run().await {
        Err(err) => {
            logs::error(err.to_string());
            exit(1)
        }
        Ok(_) => (),
    };
}

async fn run() -> Result<()> {
    // Get configuration
    let path =
        env::var("VOICELAND_CONFIG_PATH").unwrap_or("/etc/voiceland/core/config.yaml".into());

    let config = config(Path::new(&path)).await?;

    logs::ok("Config loaded");

    // Creating temp folder
    fs::create_dir_all("/tmp/voiceland").await?;

    logs::ok("Created /tmp/voiceland folder");

    // Set TLS configuration
    let server_cert: Vec<Certificate>;
    let server_key: PrivateKey;

    // If there's cert/key
    if let (Some(cert), Some(key)) = (config.tls_cert.cert, config.tls_cert.key) {
        let cert = fs::read(cert).await?;
        let key = fs::read(key).await?;

        let cert: Vec<Certificate> = certs(&mut &*cert)?.into_iter().map(Certificate).collect();

        let mut key = match pkcs8_private_keys(&mut &*key) {
            Err(_) => match rsa_private_keys(&mut &*key) {
                Err(err) => bail!(err),
                Ok(a) => a,
            },
            Ok(a) => a,
        };

        server_cert = cert;
        server_key = PrivateKey(key.remove(0));
    }
    // If there's server name
    else {
        let cert =
            rcgen::generate_simple_self_signed(vec![config.tls_cert.server_name.unwrap().into()])
                .unwrap();
        let key = cert.serialize_private_key_der();
        let cert = cert.serialize_der()?;

        server_cert = vec![Certificate(cert)];
        server_key = PrivateKey(key);
    }

    // Build TLS configuration
    let tls_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(server_cert, server_key)?;

    logs::ok("TLS certificate and key set");

    // Build server configuration
    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(tls_config));

    // Transport configuration
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();

    transport_config.keep_alive_interval(Some(Duration::from_millis(default::KEEP_ALIVE_INTERVAL)));
    transport_config.max_idle_timeout(Some(
        Duration::from_millis(default::MAX_IDLE_TIMEOUT).try_into()?,
    ));

    logs::ok("QUIC socket configuration set");
    logs::info("Finished initialization state");

    // Endpoint
    let endpoint = Endpoint::server(server_config, config.address)?;

    logs::ok(format!(
        "Server ready and listening at {}",
        endpoint.local_addr()?
    ));

    // Socket broadcast
    // TODO This needs to be restructured
    let (tx, _) = broadcast::channel::<Vec<u8>>(u8::MAX as usize);

    // Connection handler
    while let Some(conn) = endpoint.accept().await {
        let tx = tx.clone();

        tokio::spawn(async move {
            if let Err(err) = handler::handler(conn, tx).await {
                logs::error(err);
            }
        });
    }

    Ok(())
}
