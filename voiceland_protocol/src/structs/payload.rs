use super::op;

/// Payload enum with his bit mask and payload
///
/// Enum names are marked as "<operation><0 - stream | 1 - operation><hex operation mask>
#[repr(u8)]
pub enum PayloadRequest {
    OpenPortal11(op::open_portal::Request) = 1 << 7,
    JoinPortal12(op::join_portal::JoinPortal),
}
