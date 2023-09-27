/// CLIENT REQUEST - Join a portal
pub struct Request {
    /// 26 bytes
    pub id: String,
}

/// SERVER RESPONSE
pub struct Response {
    pub join: bool,
}
