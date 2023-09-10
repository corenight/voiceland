use crate::structs::config::QuicConfiguration;

// QUIC default
pub const KEEP_ALIVE_INTERVAL: u64 = 5_000;
pub const MAX_IDLE_TIMEOUT: u64 = 10_000;

pub const QUIC_CONFIG: QuicConfiguration = QuicConfiguration {
    keep_alive_interval: Some(KEEP_ALIVE_INTERVAL),
    max_idle_timeout: Some(MAX_IDLE_TIMEOUT),
};
