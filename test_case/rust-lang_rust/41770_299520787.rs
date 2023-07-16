rust
/// Compress a buffer without writing any sort of header on the output. Fast
/// compression is used because it is almost twice as fast as default
/// compression and the compression ratio is only marginally worse.
pub fn deflate_bytes(bytes: &[u8]) -> Bytes;

/// Decompress a buffer without parsing any sort of header on the input.
pub fn inflate_bytes(bytes: &[u8]) -> Result<Bytes, Error>;
