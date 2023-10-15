use std::sync::Arc;

use anyhow::Result;
use quinn::Endpoint;

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // input

    let addr = "[::1]:6050"; //inquire::Text::new("Server address").prompt()?;

    let tls_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    let config = quinn::ClientConfig::new(Arc::new(tls_config));
    //let config = quinn::ClientConfig::with_native_roots();

    let mut endpoint = Endpoint::client("[::]:0".parse()?)?;
    endpoint.set_default_client_config(config);

    let conn = endpoint.connect(addr.parse()?, "voiceland")?.await?;

    // let (tx, _) = tokio::sync::broadcast::channel::<String>(u8::MAX as usize);

    /* let tx_thr = tx.clone();
    tokio::spawn(async move {
        let mut rx = tx_thr.subscribe();
        loop {
            let mut buf = vec![0; u16::MAX as usize];

            let (mut send, mut recv) = conn.open_bi().await.unwrap();

            tokio::select! {
                msg = rx.recv()  => {
                    send.write(msg.unwrap().as_bytes()).await.unwrap();
                },


            size = recv.read(&mut buf) => {
                let buf_size = match size {
                    Err(err) => panic!("{}", err),
                    Ok(n) => match n {
                        None => break,
                        Some(m) => m
                    }
                };

                buf.resize(buf_size, 0);

                println!("{}", String::from_utf8_lossy(&buf));
            }
            }

            let res = recv.read_to_end(usize::MAX).await.unwrap();
            println!("Server dijo:: {}", String::from_utf8_lossy(&res));
        }
    });

    loop {
        let input = inquire::Text::new("Message").prompt().unwrap();

        tx.send(input)?;
    } */

    let op = inquire::Select::new("Client mode", vec!["send", "recv"]).prompt()?;

    match op {
        "recv" => loop {
            let (_, mut recv) = conn.open_bi().await.unwrap();
            let mut buf = vec![0; u16::MAX as usize];

            let size = recv.read(&mut buf).await;

            let buf_size = match size {
                Err(err) => panic!("{}", err),
                Ok(n) => match n {
                    None => break,
                    Some(m) => m,
                },
            };

            buf.resize(buf_size, 0);

            println!("{}", String::from_utf8_lossy(&buf));
        },
        "send" => loop {
            let (mut send, _) = conn.open_bi().await.unwrap();
            let input = inquire::Text::new("Message").prompt().unwrap();

            send.write(input.as_bytes()).await.unwrap();
        },
        _ => panic!("larva"),
    }

    Ok(())
}
