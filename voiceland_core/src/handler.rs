use std::pin::Pin;

use anyhow::{bail, Result};
use futures::{stream::FuturesUnordered, Future, StreamExt};
use quinn::{Connecting, ConnectionError, RecvStream, SendStream};
use tokio::sync::broadcast::Sender;
use voiceland_common::logs;

pub async fn handler(conn: Connecting, tx: Sender<Vec<u8>>) -> Result<()> {
    let conn = conn.await?;

    let mut rx = tx.subscribe();

    loop {
        let (mut send, mut recv) = match conn.accept_bi().await {
            Err(ConnectionError::ApplicationClosed { .. })
            | Err(ConnectionError::ConnectionClosed { .. })
            | Err(ConnectionError::TimedOut { .. }) => break,
            Err(err) => bail!(err),
            Ok(a) => a,
        };

        /* let mut buf = vec![0; u16::MAX as usize];
        tokio::select! {
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    println!("[  RX  ] {}", String::from_utf8_lossy(&msg));

                    send.write(msg.as_slice()).await?;
                }
            }

            size = recv.read(&mut buf) => {
                let size = match size {
                    Err(err) => bail!(err),
                    Ok(n) => match n {
                        None => break,
                        Some(m) => m,
                    },
                };

                buf.resize(size, 0);

                println!("[ QUIC ] {}", String::from_utf8_lossy(&buf));

                tx.send(buf)?;
            }
        } */

        /* tokio::select! {
            _ = broadcast(tx.clone(), &mut send) => {}
            _ = socket(tx.clone(), &mut recv) => {}
        } */

        /* let mut tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<()>> + Send>>>::new();

        tasks.push(Box::pin(broadcast(tx.clone(), &mut send)));
        tasks.push(Box::pin(socket(tx.clone(), &mut recv)));

         while let Some(Err(err)) = tasks.next().await {
            panic!("{}", err)
        } */

        let tx1 = tx.clone();
        let tx2 = tx.clone();
        tokio::spawn(async move {
            if let Err(err) = broadcast(tx1, &mut send).await {
                panic!("{}", err)
            }
        });
        tokio::spawn(async move {
            if let Err(err) = socket(tx2, &mut recv).await {
                panic!("{}", err)
            }
        });
    }

    Ok(())
}

async fn broadcast(tx: /* & */ Sender<Vec<u8>>, send: &mut SendStream) -> Result<()> {
    let mut rx = tx.subscribe();
    let msg = rx.recv().await;

    if let Ok(msg) = msg {
        println!("[  RX  ] {}", String::from_utf8_lossy(&msg));
        send.write(msg.as_slice()).await?;
    }

    Ok(())
}

async fn socket(tx: Sender<Vec<u8>>, recv: &mut RecvStream) -> Result<()> {
    let mut buf = vec![0; u16::MAX as usize];

    let size = recv.read(&mut buf).await;
    let size = match size {
        Err(err) => bail!(err),
        Ok(n) => match n {
            None => return Ok(()),
            Some(m) => m,
        },
    };

    buf.resize(size, 0);

    println!("[ QUIC ] {}", String::from_utf8_lossy(&buf));

    tx.send(buf)?;

    Ok(())
}
