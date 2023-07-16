rust
const DEFAULT_CHUNK_SIZE: usize = 65536;
[...]
pub struct Writer<'a, W, A, const C: usize = DEFAULT_CHUNK_SIZE>
[...]
