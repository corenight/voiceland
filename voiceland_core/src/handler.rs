use anyhow::Result;
use quinn::Connecting;

use crate::multiverse::Multiverse;

pub async fn init(conn: &mut Connecting, mv: &mut Multiverse) -> Result<()> {
    let conn = conn.await?;

    let recv = match conn.accept_uni().await {
        Err(ConnectionError::ApplicationClosed { .. })
        | Err(ConnectionError::ConnectionClosed { .. })
        | Err(ConnectionError::TimedOut { .. }) => panic!("Nada, se ha cerrado la conn"),
        Err(err) => panic!("{}", err),
        Ok(a) => a,
    };

    println!("{:#?}", mv.users);

    Ok(())
}
