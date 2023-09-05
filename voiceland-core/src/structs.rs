use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

// TODO finish QuicConfiguration
/// QUIC configuration structure
/// These fields represent part of Quinn configuration methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QuicConfiguration {
    /// Keep alive interval (ms)
    ///
    /// This must be lower than `max_idle_timeout`
    pub keep_alive_interval: u64,

    /// Max idle timeout (ms)
    pub max_idle_timeout: u64,
}

/// TLS certificate configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsCertConfiguration {
    /// Certificate
    pub cert: Option<String>,

    /// Key
    pub key: Option<String>,

    /// This server name must be set if no cert-key is set in order to generate self-signed certs.
    pub server_name: Option<String>,
}

/// YAML Server configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /*
     * Required data
     */
    /// Server address (with port)
    pub address: SocketAddr,

    /// TLS cert
    pub tls_cert: TlsCertConfiguration,

    /*
     * Custom data
     */
    /// QUIC basic configuration
    pub quic_conf: Option<QuicConfiguration>,
}
