/// CLIENT - Open a portal
pub struct OpenPortal {
    /// n + 1 bytes
    pub name: String,

    /// n + 1 bytes
    pub description: String,

    // TODO
    /// u8
    pub security: ,

    // u8
    pub compression: ,

    /// Audio codec
    pub audio: ,

    /// Video codec
    pub video: ,
}
