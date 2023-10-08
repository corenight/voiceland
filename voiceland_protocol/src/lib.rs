use anyhow::{bail, Result};
use parser::open_portal;
use structs::{
    packet::Packet,
    payload::{Payload, PayloadRequest, PayloadResponse},
};

pub mod parser;
pub mod structs;

pub fn to_packet(packet: Packet) -> Result<Vec<u8>> {
    let res = match packet.payload {
        Payload::Request(req) => match req {
            PayloadRequest::OpenPortal11(request) => open_portal::serialize_request(&mut request),
            _ => bail!(""),
        },
        Payload::Response(res) => match res {
            PayloadResponse::OpenPortal11(response) => {
                open_portal::serialize_response(&mut response)
            }
            _ => bail!(""),
        },
    };

    Ok(res)
}
