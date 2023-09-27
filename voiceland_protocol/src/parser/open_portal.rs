use crate::structs::op::open_portal::Request;

pub fn serialize_request(payload: &mut Request) -> Vec<u8> {
    let mut packet: Vec<u8> = vec![];

    payload.name.push('\n');
    payload.description.push('\n');

    packet.extend_from_slice(payload.name.as_bytes());
    packet.extend_from_slice(payload.description.as_bytes());

    packet.push(payload.security as u8);
    packet.push(payload.compression as u8);

    packet.extend_from_slice(&u16::to_le_bytes(payload.audio.codec));
    packet.extend_from_slice(&u16::to_le_bytes(payload.video.codec));

    packet.extend_from_slice(&u32::to_le_bytes(payload.audio.bitrate));
    packet.extend_from_slice(&u32::to_le_bytes(payload.video.bitrate));

    packet
}
