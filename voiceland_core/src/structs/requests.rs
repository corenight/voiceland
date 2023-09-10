use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User name
    name: String,

    /// User colors (avatar and secondary)
    colors: Option<[String; 2]>,

    /// Avatar
    ///
    /// In order to preserve resources, avatars will be allocated at disk rater than memory.
    /// When connecting, this will be saved at a temporal folder located at /tmp.
    /// When requesting, this will be caught from /tmp.
    avatar: Option<Vec<u8>>,
}
