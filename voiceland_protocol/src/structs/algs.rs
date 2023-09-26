#[repr(u8)]
pub enum Security {
    /// None
    NONE = 0,

    // Assymetric
    /// CRYSTAL Kyber, default algorithm
    KYBER = 1,
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

    // Symmetric
    /// ARIA, preferred symmetric algorithm
    ARIA = 20,
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
}

// NOTE
// This could be useless since there's already audio codecs that compress the audio.
// Anyway I'll give it a chance. If is not practical, I'll remove it.
#[repr(u8)]
pub enum Compression {
    /// None
    NONE = 0,

    /// ZStandard, default algorithm
    ZSTD = 1,
    BROTLI,
    ZLIB,
    SNAPPY,
    GZIP,
    DEFLATE,
    LZ77,
    LZSS,
    LZMA,
    LZ4,
}
