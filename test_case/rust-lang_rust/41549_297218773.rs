rust
// In library
pub trait ManagedChunk: Serialize + Deserialize {
    const SECTOR_SIZE: usize = 4096;

    const REGION_WIDTH: i32 = 16;
}

// ...

// In binary project
impl ManagedChunk for SerialChunk {
    const SECTOR_SIZE: u32 = 4096;

    const REGION_WIDTH: u32 = 16;
}
