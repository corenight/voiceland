//! # Voiceland protocol parser
//! This module receives Voiceland packets and parses it, reading operation header and parsing required data for each operation.

use anyhow::{bail, Result};

use num_traits::FromPrimitive;
use structs::Packet;

mod serializers;
pub mod structs;

/* pub fn to_packet(input: InputPacket) -> Result<Vec<u8>> {

} */

pub fn from_packet(input: &mut Vec<u8>) -> Result<Packet> {
    if input.len() == 0 {
        bail!("No data given.")
    }

    // TODO Stream
    if input[0] | 1 << 7 == 0 {
        bail!("Not supported yet")
    }
    // Operations
    else {
        let data = match FromPrimitive::from_u8(input[0]) {
            Some(a) => match a {
                structs::BitMask::CreatePortal => serializers::open_portal_11(input)?,
                _ => bail!("Unknown operation"),
            },
            None => bail!("Cannot parse bit mask"),
        };

        Ok(data)
    }
}
