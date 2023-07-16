rust
fn read_to_uninitialized(&mut self, buf: impl UninitializedBuffer<u8>) -> Result<usize>;
