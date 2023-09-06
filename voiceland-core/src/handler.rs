use anyhow::{bail, Result};
use quinn::{Connecting, ConnectionError};
use tokio::sync::broadcast::Sender;

pub async fn handler(conn: Connecting, buf: usize, tx: Sender<Vec<u8>>) -> Result<()> {
    let conn = conn.await?;

    loop {
        let (mut send, mut recv) = match conn.accept_bi().await {
            Err(ConnectionError::ApplicationClosed { .. })
            | Err(ConnectionError::ConnectionClosed { .. }) => break,
            Err(err) => bail!(err),
            Ok(a) => a,
        };

        let mut buf = vec![0; buf];

        tokio::select! {
            size = recv.read(&mut buf) => {
                let buf_size = match size {
                    Err(err) => bail!(err),
                    Ok(n) => match n {
                        None => break,
                        Some(m) => m
                    }
                };

                buf.resize(buf_size, 0);

                print!("{}", String::from_utf8_lossy(&buf));
            }
        }
    }

    Ok(())
}
