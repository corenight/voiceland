//! # Voiceland protocol parser
//! This module receives Voiceland packets and parses it, reading operation header and parsing required data for each operation.

use anyhow::{bail, Result};
use bincode;

use structs::Packet;

mod serializers;
pub mod structs;

/* pub fn to_packet(input: InputPacket) -> Result<Vec<u8>> {

} */

pub fn from_packet(input: &mut Vec<u8>) -> Result<Packet> {
    if input.len() == 0 {
        bail!("No data given.")
    }

    // Stream
    if input[0] | 1 << 7 == 0 {
        bail!("Not supported yet")
    }
    // Operations
    else {
        let data = match input[0] as structs::BitMask {
            structs::BitMask::CreatePortal => serializers::open_portal_11(&mut input)?,
            _ => bail!("Unknown operation"),
        };

        Ok(data)
    }
}
