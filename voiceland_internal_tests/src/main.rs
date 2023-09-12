use std::ffi::CString;

use rsmpeg::avcodec;
use voiceland_protocol::{
    from_packet,
    structs::{self, InputPacket},
    to_packet,
};

fn main() {
    let name = "libopus";

    let a = avcodec::AVCodec::find_encoder_by_name(&CString::new(name).unwrap()).unwrap();

    println!("{:#?}", a.long_name());

    let data = InputPacket {
        op: 0b1000000,
        payload: structs::Payload::ClientOpenPortal(structs::ClientOpenPortal {
            name: "larva".into(),
            security: structs::AlgorithmNumber::ARIA,
            audio: structs::AudioClientOpenPortal {
                codec: 12,
                bitrate: 64000,
            },
            video: structs::VideoClientOpenPortal {
                codec: 14,
                bitrate: 1000000,
            },
        }),
    };

    let bitos = to_packet(data).unwrap();

    println!("{:#?}", String::from_utf8_lossy(&bitos));
    println!("{:#?}", from_packet(bitos));
}
