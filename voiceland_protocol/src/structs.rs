use 

/// Header bit mask
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

    B3 = 1 << 3,
    B4 = 1 << 4,
    B5 = 1 << 5,
    B6 = 1 << 6,
    B7 = 1 << 7,
}

/// Security algorithms
///
/// This could change because I don't decided yet what library use for security.
/// These algorithms are most important at RustCrypto
pub enum AlgorithmNumber {
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

/// Codec enum

/// Data enum
pub enum Payload {
    Op1JoinPortal,
}

/// CLIENT - Join a portal
pub struct ClientJoinPortal {
    /// 26 bytes
    id: String,
}

/// CLIENT - Open a portal
pub struct ClientOpenPortal {
    /// 32 bytes
    name: String,

    /// u8
    security: AlgorithmNumber,

    /// Audio codec
    audio_codec: ,
    audio_bitrate: 
}

/// Parse to packet
pub struct InputPacket {
    op: u8,
    payload: Payload,
}
