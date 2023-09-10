//! ## Avatar allocator
//! Avatar disk allocator. This aims to avoid memory usage for store avatars.

use std::path::Path;

use anyhow::Result;
use tokio::fs;
use ulid::Ulid;

fn path<'a>() -> &'a Path {
    Path::new("/tmp/voiceland")
}

/// Allocate image (with compression)
pub async fn alloc(buf: Vec<u8>) -> Result<()> {
    fs::write(path().join(Ulid::new().to_string()), buf).await?;

    Ok(())
}

/// Get image
pub async fn get(ulid: String) -> Result<Vec<u8>> {
    let data = fs::read(path().join(ulid)).await?;

    Ok(data)
}

/// Deallocate image
pub async fn dealloc(ulid: String) -> Result<()> {
    fs::remove_file(path().join(ulid)).await?;

    Ok(())
}
