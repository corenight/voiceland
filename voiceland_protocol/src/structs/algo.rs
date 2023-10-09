/*
https://github.com/brndnmtthws/dryoc

I need to have a better algorithm list
*/

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Security {
    /// None
    NONE = 0,

    // Asymmetric (First - 1)
    // There's not NIST P-521 because his RustCrypto lib is still in development
    KYBER,          // https://github.com/Argyle-Software/kyber
    CURVE25519,     // https://github.com/dalek-cryptography/curve25519-dalek
    RSA,            // https://github.com/RustCrypto/RSA
    SECP256K1,      // https://github.com/RustCrypto/elliptic-curves/blob/master/k256
    BIGNCURVE256V1, // https://github.com/RustCrypto/elliptic-curves/tree/master/bign256
    NISTP192,       // https://github.com/RustCrypto/elliptic-curves/blob/master/p192
    NISTP224,       // https://github.com/RustCrypto/elliptic-curves/blob/master/p224
    NISTP256,       // https://github.com/RustCrypto/elliptic-curves/blob/master/p256
    NISTP384,       // https://github.com/RustCrypto/elliptic-curves/blob/master/p384
    SM2,            // https://github.com/RustCrypto/elliptic-curves/blob/master/sm2
    MNT4_298,       // https://github.com/arkworks-rs/curves/blob/master/mnt4_298
    MNT6_298,       // https://github.com/arkworks-rs/curves/blob/master/mnt6_298
    MNT4_753,       // https://github.com/arkworks-rs/curves/blob/master/mnt4_753
    MNT6_753,       // https://github.com/arkworks-rs/curves/blob/master/mnt6_753

    // Symmetric (first - 50)
    ARIA = 50,        // https://github.com/RustCrypto/block-ciphers/blob/master/aria
    AES,              // https://github.com/RustCrypto/AEADs/tree/master/aes-gcm-siv
    CHACHA20POLY1305, // https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305
    BELT,             // https://github.com/RustCrypto/block-ciphers/blob/master/belt-block
    BLOWFISH,         // https://github.com/RustCrypto/block-ciphers/blob/master/blowfish
    CAMELLIA,         // https://github.com/RustCrypto/block-ciphers/blob/master/camellia
    CAST5,            // https://github.com/RustCrypto/block-ciphers/blob/master/cast5
    DES3DES,          // https://github.com/RustCrypto/block-ciphers/blob/master/des
    IDEA,             // https://github.com/RustCrypto/block-ciphers/blob/master/idea
    KUZNYECHIK,       // https://github.com/RustCrypto/block-ciphers/blob/master/kuznyechik
    MAGMA,            // https://github.com/RustCrypto/block-ciphers/blob/master/magma
    RC2,              // https://github.com/RustCrypto/block-ciphers/blob/master/rc2
    RC5,              // https://github.com/RustCrypto/block-ciphers/blob/master/rc5
    SERPENT,          // https://github.com/RustCrypto/block-ciphers/blob/master/serpent
    SM4,              // https://github.com/RustCrypto/block-ciphers/blob/master/sm4
    TWOFISH,          // https://github.com/RustCrypto/block-ciphers/blob/master/twofish
    THREEFISH,        // https://github.com/RustCrypto/block-ciphers/blob/master/threefish
}

// NOTE
// This could be useless since there's already audio codecs that compress the audio.
// Anyway I'll give it a chance. If is not practical, I'll remove it.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Compression {
    /// None
    NONE = 0,

    ZSTD = 1, // https://github.com/gyscos/zstd-rs
    BROTLI,   // https://github.com/dropbox/rust-brotli
    ZLIB,     // https://github.com/etemesi254/zune-image/tree/main/zune-inflate
    DEFLATE,  // - Same as Zlib
    GZIP,     // - Same as Zlib
    SNAPPY,   // https://github.com/BurntSushi/rust-snappy
    LZ77,     // https://github.com/sile/libflate
    LZSS,     // https://github.com/alexkazik/lzss
    LZMA,     // https://github.com/fpgaminer/rust-lzma
    LZ4,      // https://github.com/10xGenomics/lz4-rs
}
