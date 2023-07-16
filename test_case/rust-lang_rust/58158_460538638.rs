rust
#[repr(packed)]
pub struct Foo(<Vec<u32> as IntoIterator>::Item);
