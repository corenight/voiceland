//! # Voiceland protocol parser
//! This module receives Voiceland packets and parses it, reading operation header and parsing required data for each operation.

use anyhow::Result;
use bincode;
use structs::InputPacket;

use crate::structs::OpBitMask;

pub mod structs;

pub fn to_packet(input: InputPacket) -> Result<Vec<u8>> {
    let data = bincode::serialize(&input)?;

    Ok(data)
}

pub fn from_packet(input: Vec<u8>) -> Result<InputPacket> {
    let data = bincode::deserialize::<InputPacket>(&input)?;

    Ok(data)
}

#[test]
fn serialize_test() {
    assert_eq!(1, 1)
}
