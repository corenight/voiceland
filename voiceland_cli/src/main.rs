// PLEASE
// Ignore the code. Is messy but it works.
// Is just a weird test.

use std::{
    sync::{mpsc, Arc},
    time::Duration,
};

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
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
    let addr = "127.0.0.1:6050"; //inquire::Text::new("Server address").prompt()?;

    let tls_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    let config = quinn::ClientConfig::new(Arc::new(tls_config));

    let mut endpoint = Endpoint::client("[::]:0".parse()?)?;
    endpoint.set_default_client_config(config);

    /***********************************/

    let host = cpal::default_host();

    let input = host.default_input_device().unwrap();
    let output = host.default_output_device().unwrap();

    let config: cpal::StreamConfig = input.default_input_config().unwrap().into();

    let err = |err: cpal::StreamError| panic!("{}", err);

    /***********************************/

    let conn = endpoint.connect(addr.parse()?, "voiceland")?.await?;

    let (mut send, mut recv) = conn.open_bi().await.unwrap();

    // This sends a packet to notify server that stream has been opened
    send.write(b"").await?;

    let (mut recv_tx, mut recv_rx) = mpsc::channel::<Vec<u8>>();

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

            recv_tx.send(buf.clone()).unwrap();
        }
    });

    /***********************************/

    let output_stream = output
        .build_output_stream(
            &config,
            move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                let mut broma = recv_rx.recv().unwrap();
                for sample in data {
                    *sample = broma.pop().unwrap_or(0);
                }
            },
            err,
            None,
        )
        .unwrap();

    let (mut send_tx, mut send_rx) = mpsc::channel::<Vec<u8>>();

    let input_stream = input
        .build_input_stream(
            &config,
            move |data: &[u8], _: &cpal::InputCallbackInfo| {
                let data = data.to_vec();

                send_tx.send(data).unwrap();
            },
            err,
            None,
        )
        .unwrap();

    input_stream.play().unwrap();
    output_stream.play().unwrap();

    loop {
        let data = send_rx.recv().unwrap();

        send.write(data.as_slice()).await.unwrap();
    }

    Ok(())
}
