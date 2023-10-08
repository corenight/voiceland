use anyhow::{bail, Result};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::{
    structs::{self, CompressionAlgNumber, SecurityAlgNumber},
    utils,
};

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

    let name = String::from_utf8_lossy(&buffer.next().unwrap().to_vec()).to_string();
    let description = String::from_utf8_lossy(&buffer.next().unwrap().to_vec()).to_string();

    let data = buffer.next().unwrap();

    let security: SecurityAlgNumber = unsafe { std::mem::transmute(data[0]) };
    let compression: CompressionAlgNumber = unsafe { std::mem::transmute(data[1]) };
    let audio_codec = utils::to_u32(data[2..=5].try_into()?)?;
    let video_codec = utils::to_u16(data[6..=7].try_into()?)?;
    let audio_bitrate = utils::to_u64(data[8..=11].try_into()?)?;
    let video_bitrate = utils::to_u64(data[12..=15].try_into()?)?;

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
                codec: video_codec as u32,
                bitrate: video_bitrate,
            },
        }),
    })
}
