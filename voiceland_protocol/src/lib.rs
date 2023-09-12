//! # Voiceland protocol parser
//! This module receives Voiceland packets and parses it, reading operation header and parsing required data for each operation.

use anyhow::{bail, Result};
use bincode;
use structs::Packet;

pub mod structs;

/* pub fn to_packet(input: InputPacket) -> Result<Vec<u8>> {

} */

pub fn from_packet(input: Vec<u8>) -> Result<Packet> {
    if input.len() == 0 {
        bail!("No data given.")
    }

    let data: Packet;

    // Finish tomorrow
    match input.as_slice() {
        [0] => ,
        _ => (),
    }
}
