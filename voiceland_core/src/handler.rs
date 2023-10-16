use core::panic;

use anyhow::{bail, Result};
use futures::stream::FuturesUnordered;
use quinn::{Connecting, ConnectionError, RecvStream, SendStream};
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

    loop {
        let (mut send, mut recv) = match conn.accept_bi().await {
            Err(ConnectionError::ApplicationClosed { .. })
            | Err(ConnectionError::ConnectionClosed { .. })
            | Err(ConnectionError::TimedOut { .. }) => break,
            Err(err) => bail!(err),
            Ok(a) => a,
        };

        let tx1 = tx.clone();
        let tx2 = tx.clone();
        tokio::spawn(async move {
            if let Err(err) = broadcast(tx1, &mut send).await {
                panic!(
                    "El tokio::sync::broadcast se queda más colgado que un llavero\n{}",
                    err
                )
            }
        });

        tokio::spawn(async move {
            if let Err(err) = recv_packet(tx2, &mut recv).await {
                panic!(
                    "El quinn::RecvStream se queda más colgado que un llavero\n{}",
                    err
                )
            }
        });
    }

    Ok(())
}

async fn broadcast(tx: Sender<Vec<u8>>, send: &mut SendStream) -> Result<()> {
    let mut rx = tx.subscribe();

    let msg = rx.recv().await;

    if let Ok(msg) = msg {
        println!("[ MPMC ] {}", String::from_utf8_lossy(&msg));
        send.write(msg.as_slice()).await?;
    }

    Ok(())
}

async fn recv_packet(tx: Sender<Vec<u8>>, recv: &mut RecvStream) -> Result<()> {
    let mut buf = vec![0; u16::MAX as usize];

    let size = recv.read(&mut buf).await;

    let size = match size {
        Err(err) => panic!("{}", err),
        Ok(n) => match n {
            None => return Ok(()),
            Some(m) => m,
        },
    };

    buf.resize(size, 0);

    println!("[ RECV ] {}", String::from_utf8_lossy(&buf));

    tx.send(buf).unwrap();

    Ok(())
}
