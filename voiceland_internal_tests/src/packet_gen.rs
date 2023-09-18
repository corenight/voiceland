// Really professional test, generating manually packets

fn main() {
    // Packet 1 << 7 + 1

    /*
    String name           // n >= 3 && n <= 32 + 1 bytes
    String description    // n >= 3 && m <= 128 + 1 bytes
    u8 security           // 1 byte
    u8 compression        // 1 byte
    u32 audio codec        // 4 bytes
    u16 video codec        // 2 bytes
    u32 audio bitrate     // 4 bytes
    u32 video bitrate     // 4 bytes
    */

    let name = "Sas profunda\n".as_bytes();
    let description = "Hello I like sasilla frescota and bugs\n".as_bytes();

    let security = 0u8; // ARIA
    let compression = 0u8; // ZSTD

    let audio_codec = 86028u32; // FLAC (as FFmpeg says)
    let video_codec = 167u16; // AV1?

    let audio_bitrate = 128_000u32;
    let video_bitrate = 1_000_000u32;

    let mut packet: Vec<u8> = vec![];

    packet.extend_from_slice(name);
    packet.extend_from_slice(description);

    packet.push(security);
    packet.push(compression);

    // boilerplate, I don't want to create something productive
    for i in (0..4).rev() {
        let byte = (audio_codec >> (i * 8)) as u8;
        packet.push(byte);
    }
    for i in (0..2).rev() {
        let byte = (video_codec >> (i * 8)) as u8;
        packet.push(byte);
    }

    for i in (0..4).rev() {
        let byte = (audio_bitrate >> (i * 8)) as u8;
        packet.push(byte);
    }

    for i in (0..4).rev() {
        let byte = (video_bitrate >> (i * 8)) as u8;
        packet.push(byte);
    }

    println!("{:?}", packet);
    println!("{:?}", String::from_utf8_lossy(&packet));
    /*[83, 97, 115, 32, 112, 114, 111, 102, 117, 110, 100, 97, 72, 101, 108, 108, 111, 32, 73, 32, 108, 105, 107, 101, 32, 115, 97, 115, 105, 108, 108, 97, 32, 102, 114, 101, 115, 99, 111, 116, 97, 32, 97, 110, 100, 32, 98, 117, 103, 115, 0, 0, 0, 1, 80, 12, 0, 167, 0, 1, 244, 0, 0, 15, 66, 64]*/
}
