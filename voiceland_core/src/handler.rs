use std::net::SocketAddr;

use anyhow::{bail, Result};
use quinn::{Connecting, ConnectionError};
use tokio::sync::broadcast;

pub async fn init(
    conn: &mut Connecting,
    mv: &mut broadcast::Sender<(SocketAddr, Vec<u8>)>,
) -> Result<()> {
    let conn = conn.await?;

    let (mut send, mut recv) = match conn.accept_bi().await {
        Err(ConnectionError::ApplicationClosed { .. })
        | Err(ConnectionError::ConnectionClosed { .. })
        | Err(ConnectionError::TimedOut { .. }) => panic!("Nada, se ha cerrado la conn"),
        Err(err) => panic!("{}", err),
        Ok(a) => a,
    };

    let mut rx = mv.subscribe();

    loop {
        let mut buf = vec![0u8; u16::MAX as usize];
        tokio::select! {
            msg = recv.read(&mut buf) => {
                let buf_size = match msg {
                    Err(err) => bail!(err),
                    Ok(n) => match n {
                        None => break,
                        Some(m) => m
                    }
                };
                buf.resize(buf_size, 0);

                mv.send((conn.remote_address(), buf))?;
            }

            msg = rx.recv() => {
                if let Ok((addr, data)) = msg {
                    if addr != conn.remote_address() {
                        send.write(&data).await?;
                    }
                }
            }
        };
    }

    Ok(())
}
