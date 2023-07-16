rust
fn u64_chunks(bytes: &[u8]) -> Result<&[[u8; u64]]> {
    bytes.try_into().map_err(|_| SomeError)
}
// vs
fn u64_chunks(bytes: &[u8]) -> Result<&[[u8; u64]]> {
    let (chunks, []) = bytes.as_chunks() else { return Err(SomeError) };
    chunks
}
