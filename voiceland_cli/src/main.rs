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

    let mut endpoint = Endpoint::client("[::]:0".parse()?)?;
    endpoint.set_default_client_config(config);

    let conn = endpoint.connect(addr.parse()?, "voiceland")?.await?;

    let (mut send, mut recv) = conn.open_bi().await.unwrap();

    // This sends a packet to notify server that stream has been opened
    send.write(b"").await?;

    tokio::spawn(async move {
        loop {
            let mut buf = vec![0; u16::MAX as usize];

            let size = recv.read(&mut buf).await;

            let buf_size = match size {
                Err(err) => panic!("{}", err),
                Ok(n) => match n {
                    None => return,
                    Some(m) => m,
                },
            };

            buf.resize(buf_size, 0);

            println!("{}", String::from_utf8_lossy(&buf));
        }
    });

    // let mut send = conn.open_uni().await?;

    loop {
        let input = inquire::Text::new("Message").prompt().unwrap();

        send.write(input.as_bytes()).await?;
    }

    Ok(())
}
