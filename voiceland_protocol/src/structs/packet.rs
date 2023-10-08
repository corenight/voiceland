use super::payload::Payload;

#[repr(u8)]
pub enum Op {
    IDK = 0b1000_0000,
    OpenPortal,
    JoinPortal,
}

pub struct Packet {
    pub op: Op,
    pub payload: Payload,
}
