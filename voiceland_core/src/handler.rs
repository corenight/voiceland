use anyhow::{bail, Result};
use quinn::{Connecting, Connection, ConnectionError, RecvStream, SendStream};
use tokio::sync::broadcast::{Receiver, Sender};

pub async fn handler(conn: Connecting, tx: Sender<Vec<u8>>, tx_join: Sender<u8>) -> Result<()> {
    let conn = conn.await?;

    tokio::spawn(async move {});

    Ok(())

    /* let (mut send, mut recv) = match conn.accept_bi().await {
        Err(ConnectionError::ApplicationClosed { .. })
        | Err(ConnectionError::ConnectionClosed { .. })
        | Err(ConnectionError::TimedOut { .. }) => return Ok(()),
        Err(err) => {
            bail!(err)
        }
        Ok(a) => a,
    };

    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            _ = socket(&mut recv, &tx) => {}
            _ = broadcast(&mut rx, &mut send) => {}
        }
    } */
}

async fn socket(conn: Connection, tx: &Sender<Vec<u8>>, tx_join: Sender<u8>) -> Result<()> {
    let mut recv = match conn.accept_uni().await {
        Err(ConnectionError::ApplicationClosed { .. })
        | Err(ConnectionError::ConnectionClosed { .. })
        | Err(ConnectionError::TimedOut { .. }) => panic!("Nada, se ha cerrado la conn"),
        Err(err) => panic!("{}", err),
        Ok(a) => a,
    };

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
