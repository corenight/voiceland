use anyhow::{bail, Result};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::structs::{self, CompressionAlgNumber, SecurityAlgNumber};

/// Operation - Open portal
///
/// Open a portal. Operation 0 0 0 0 1
pub fn open_portal_11(buf: &mut Vec<u8>) -> Result<structs::Packet> {
    if buf.len() < 24 {
        bail!("Too short")
    }

    // operation
    let op = match buf.pop() {
        None => bail!("No element at vector"),
        Some(a) => a,
    };

    let mut buffer = buf.split(|a| a == &10u8);

    let mut name = String::from_utf8_lossy(&buffer.next().unwrap().to_vec()).to_string();
    let mut description = String::from_utf8_lossy(&buffer.next().unwrap().to_vec()).to_string();

    let [security, compression, audio_codec @ 4, video_codec @ 4, audio_bitrate @ 4, video_bitrate @ 4] =
        buffer.next().unwrap();

    let security: SecurityAlgNumber = unsafe { std::mem::transmute(security) };
    let compression: CompressionAlgNumber = unsafe { std::mem::transmute(compression) };

    Ok(structs::Packet {
        op,
        payload: structs::Payload::OpenPortal11(structs::OpenPortal11 {
            name,
            description,
            security,
            compression,
            audio: structs::Audio {
                codec: audio_codec,
                bitrate: audio_bitrate,
            },
            video: structs::Video {
                codec: video_codec,
                bitrate: video_bitrate,
            },
        }),
    })
}
