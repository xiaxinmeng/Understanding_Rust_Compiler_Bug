rust
pub type MetadataRef = OwningRef<Box<dyn Erased + Send + Sync>, [u8]>;
