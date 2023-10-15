use anyhow::{bail, Result};
use futures::stream::FuturesUnordered;
use quinn::{Connecting, ConnectionError};
use tokio::sync::broadcast::Sender;
use voiceland_common::logs;

pub async fn handler(conn: Connecting, tx: Sender<Vec<u8>>) -> Result<()> {
    /* let conn = conn.await?;

    let mut rx = tx.subscribe();

    loop {
        let (mut send, mut recv) = match conn.accept_bi().await {
            Err(ConnectionError::ApplicationClosed { .. })
            | Err(ConnectionError::ConnectionClosed { .. })
            | Err(ConnectionError::TimedOut { .. }) => break,
            Err(err) => bail!(err),
            Ok(a) => a,
        };

        let mut buf = vec![0; u16::MAX as usize];

        let tasks = FuturesUnordered::new();

        tasks.push({
            let size = recv.read(&mut buf).await;

            let size = match size {
                Err(err) => bail!(err),
                Ok(n) => match n {
                    None => break,
                    Some(m) => m,
                },
            };

            buf.resize(size, 0);

            println!("[ RECV ] {}", String::from_utf8_lossy(&buf));

            tx.send(buf)?;
        });

        tasks.push({
            let msg = rx.recv().await;

            if let Ok(msg) = msg {
                println!("[ MPMC ] {}", String::from_utf8_lossy(&msg));
                send.write(msg.as_slice()).await?;
            }
        });
    }

    logs::info(format!("{} closed the connection", conn.remote_address()));

    Ok(()) */

    let conn = conn.await?;

    let mut rx = tx.subscribe();

    tx.send(b"larva".to_vec())?;

    tokio::select! {
        msg = rx.recv() => {
            if let Ok(msg) = msg {
                println!("[ MPMC ] {}", String::from_utf8_lossy(&msg));
            }
        }
    }

    Ok(())
}
