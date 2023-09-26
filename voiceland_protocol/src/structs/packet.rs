use super::payload::Payload;

pub struct Packet {
    op: u8,
    payload: Payload,
}
