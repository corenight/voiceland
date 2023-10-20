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

    println!("{:#?}", conn.handshake_data());

    loop {
        let mut buf = vec![0; u16::MAX as usize];

        let len = match recv.read(&mut buf).await? {
            None => break,
            Some(a) => a,
        };

        buf.resize(len, 0);

        println!("{}", String::from_utf8_lossy(&buf));

        buf.reverse();

        send.write(buf.as_slice()).await.unwrap();
    }

    Ok(())
}
