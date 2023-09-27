use crate::structs::algo::{Compression, Security};

/// CLIENT REQUEST - Open a portal
pub struct Request {
    /// n + 1 bytes
    pub name: String,

    /// n + 1 bytes
    pub description: String,

    /// u8
    pub security: Security,

    /// NOTE This can be deprecated
    /// u8
    pub compression: Compression,

    /// Audio codec
    pub audio: Audio,

    /// Video codec
    pub video: Video,
}

/// SERVER RESPONSE - Return ULID
pub struct Response {
    /// 26 bytes
    pub id: String,
}

/// Audio codec
pub struct Audio {
    /// u16
    pub codec: u16,

    /// u32
    pub bitrate: u32,
}

// Video codec
pub struct Video {
    /// u16
    pub codec: u16,
    /// u32
    pub bitrate: u32,
}
