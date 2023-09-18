use num_derive::{FromPrimitive, ToPrimitive};
use rsmpeg::avcodec;
use serde::{Deserialize, Serialize};

/// Header bit mask
#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum OpBitMask {
    /// - 0: Data stream
    /// - 1: Operation stream
    KIND = 1,

    /// - 0: Ok
    /// - 1: Error
    RESPONSE = 1 << 1,

    /// - 0: No payload is given
    /// - 1: Payload is given
    PAYLOAD = 1 << 2,

    OP1 = 1 << 4,
    OP0 = 1 << 3,
    OP2 = 1 << 5,
    OP3 = 1 << 6,
    OP4 = 1 << 7,
}

/// Operation bitmask definition
#[derive(FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum BitMask {
    // Stream
    Stream = 0,

    // Operation
    Operation = 1 << 7,
    CreatePortal,
    JoinPortal,
}

/// Security algorithms
///
/// This could change because I don't decided yet what library use for security.
/// These algorithms are most important at RustCrypto
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum SecurityAlgNumber {
    // Symmetric
    ARIA,
    AES,
    BELT,
    BLOWFISH,
    CAMELLIA,
    CAST5,
    DES3DES,
    IDEA,
    KUZNYECHIK,
    MAGMA,
    RC2,
    RC5,
    SERPENT,
    SM4,
    SPECK,
    TWOFISH,
    THREEFISH,

    // Assymetric
    RSA,
    BIGNCURVE256V1,
    BRAINPOOLP256R1,
    BRAINPOOLP384R1,
    SECP256K1,
    NISTP192,
    NISTP224,
    NISTP256,
    NISTP384,
    NISTP521,
    SM2,
}

/// Compression algorithm
/// TODO add more algorithms
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum CompressionAlgNumber {
    ZSTD,
    ZLIB,
    BROTLI,
    LZ77,
    LZSS,
    SNAPPY,
    LZMA,
    G711,
}

/// Data enum
///
/// Enum names are marked as "<operation><0 - stream | 1 - operation><hex operation mask>
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    OpenPortal11(OpenPortal11),
    JoinPortal12(JoinPortal12),
}

/// CLIENT - Join a portal
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinPortal12 {
    /// 26 bytes
    pub id: String,
}

/// Audio ClientOpenPortal struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {
    pub codec: avcodec::AVCodecID,
    pub bitrate: u64,
}

/// Audio ClientOpenPortal struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub codec: avcodec::AVCodecID,
    pub bitrate: u64,
}

/// CLIENT - Open a portal
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenPortal11 {
    /// n + 1 bytes
    pub name: String,

    /// n + 1 bytes
    pub description: String,

    /// u8
    pub security: SecurityAlgNumber,

    // u8
    pub compression: CompressionAlgNumber,

    /// Audio codec
    pub audio: Audio,

    /// Video codec
    pub video: Video,
}

/// Parse to packet
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub op: u8,
    pub payload: Payload,
}
