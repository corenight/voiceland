use std::pin::Pin;

use anyhow::{bail, Result};
use futures::{stream::FuturesUnordered, Future, StreamExt};
use quinn::{Connecting, ConnectionError, RecvStream, SendStream};
use tokio::sync::broadcast::{Receiver, Sender};
use voiceland_common::logs;

pub async fn handler(conn: Connecting, tx: Sender<Vec<u8>>) -> Result<()> {
    let conn = conn.await?;

    let mut rx = tx.subscribe();

    let (mut send, mut recv) = match conn.accept_bi().await {
        Err(ConnectionError::ApplicationClosed { .. })
        | Err(ConnectionError::ConnectionClosed { .. })
        | Err(ConnectionError::TimedOut { .. }) => return Ok(()),
        Err(err) => bail!(err),
        Ok(a) => a,
    };

    loop {
        let mut buf = vec![0; u16::MAX as usize];

        tokio::select! {
            len = recv.read(&mut buf) => {
                let len = match len? {
                    None => break,
                    Some(a) => a,
                };

                buf.resize(len, 0);

                println!("[ QUIC ] {}", String::from_utf8_lossy(&buf));

                tx.send(buf)?;
            }

            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    println!("[  RX  ] {}", String::from_utf8_lossy(&msg));
                    send.write(&msg).await?;
                }
            }
        }
    }

    Ok(())
}
