use anyhow::{bail, Result};
use quinn::{Connecting, Connection, ConnectionError, RecvStream, SendStream};
use tokio::sync::broadcast::{Receiver, Sender};
use voiceland_common::logs;

pub async fn handler(conn: Connecting, tx: Sender<Vec<u8>>, tx_join: Sender<u8>) -> Result<()> {
    let conn = conn.await?;

    let tx_join_thr = tx_join.clone();
    let tx_thr = tx.clone();
    let conn_thr = conn.clone();
    tokio::spawn(async move {
        let mut recv = match conn_thr.accept_uni().await {
            Err(ConnectionError::ApplicationClosed { .. })
            | Err(ConnectionError::ConnectionClosed { .. })
            | Err(ConnectionError::TimedOut { .. }) => panic!("Nada, se ha cerrado la conn"),
            Err(err) => panic!("{}", err),
            Ok(a) => a,
        };

        while let Err(err) = socket(&mut recv, &tx_thr, &tx_join_thr).await {
            logs::error(err);
        }
    });

    let mut rx_join = tx_join.subscribe();

    loop {
        let op = rx_join.recv().await?;

        let conn = conn.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut rx = tx.subscribe();

            let mut send = match conn.open_uni().await {
                Err(ConnectionError::ApplicationClosed { .. })
                | Err(ConnectionError::ConnectionClosed { .. })
                | Err(ConnectionError::TimedOut { .. }) => panic!("Nada, se ha cerrado la conn"),
                Err(err) => panic!("{}", err),
                Ok(a) => a,
            };

            loop {
                let msg = rx.recv().await.unwrap();

                send.write(&msg.as_slice()).await.unwrap();
            }
        });
    }

    Ok(())
}

async fn socket(recv: &mut RecvStream, tx: &Sender<Vec<u8>>, tx_join: &Sender<u8>) -> Result<()> {
    tx_join.send(0x0)?;

    let mut buf = vec![0; u16::MAX as usize];

    let len = recv.read(&mut buf).await?;

    let len = match len {
        None => return Ok(()),
        Some(a) => a,
    };

    buf.resize(len, 0);

    println!("[ QUIC ] {}", String::from_utf8_lossy(&buf));

    tx.send(buf)?;
    Ok(())
}

async fn broadcast(rx: &mut Receiver<Vec<u8>>, send: &mut SendStream) -> Result<()> {
    let msg = rx.recv().await;

    if let Ok(msg) = msg {
        println!("[  RX  ] {}", String::from_utf8_lossy(&msg));
        send.write(&msg).await?;
    }

    Ok(())
}
