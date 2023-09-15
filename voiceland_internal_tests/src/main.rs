use std::ffi::CString;

use rsmpeg::avcodec;

use voiceland_protocol::{
    from_packet,
    structs::{self, Packet},
    to_packet,
};

mod packet_gen;

fn main() {
    let name = "libopus";

    let a = avcodec::AVCodec::find_encoder_by_name(&CString::new(name).unwrap()).unwrap();

    println!("{:#?}", a.long_name());

    let data = Packet {
        op: 0b1000000,
        payload: structs::Payload::OpenPortal11(structs::OpenPortal11 {
            name: "larva".into(),
            description: "menuda larvota frescota".to_string(),
            security: structs::SecurityAlgNumber::ARIA,
            compression: structs::CompressionAlgNumber::ZSTD,
            audio: structs::Audio {
                codec: 12,
                bitrate: 64000,
            },
            video: structs::Video {
                codec: 14,
                bitrate: 1000000,
            },
        }),
    };

    let raw: [u8; 66] = [
        83, 97, 115, 32, 112, 114, 111, 102, 117, 110, 100, 97,
        72, 101, 108, 108, 111, 32, 73, 32, 108, 105, 107, 101,
        32, 115, 97, 115, 105, 108, 108, 97, 32, 102, 114, 101,
        115, 99, 111, 116, 97, 32, 97, 110, 100, 32, 98, 117,
        103, 115, 0, 0, 0, 1, 80, 12, 0, 167, 0, 1,
        244, 0, 0, 15, 66, 64
    ];

    let packet = from_packet(&mut raw.to_vec()).unwrap();
    let data = match packet.payload {
        structs::Payload::OpenPortal11(a) => a,
        _ => panic!("xd")
    };

    println!("{:#?}", packet);
    println!("{}", data.name);


    /*let bitos = from_packet(data).unwrap(); // Hello I like sasilla frescota and bugs

    println!("{:#?}", String::from_utf8_lossy(&bitos));
    println!("{:#?}", from_packet(bitos));*/
}
