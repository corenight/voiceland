use anyhow::{bail, Result};

pub fn to_u16(buf: [u8; 2]) -> Result<u16> {
    if buf.len() < 2 {
        bail!("Buffer too short")
    };

    Ok(u16::from_le_bytes(buf))
}

pub fn to_u32(buf: [u8; 4]) -> Result<u32> {
    if buf.len() < 2 {
        bail!("Buffer too short")
    };

    Ok(u32::from_le_bytes(buf))
}

pub fn to_u64(buf: [u8; 8]) -> Result<u64> {
    if buf.len() < 2 {
        bail!("Buffer too short")
    };

    Ok(u64::from_le_bytes(buf))
}
