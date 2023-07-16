 rust
fn copy<R: Read, W: Write>(r: R, w: W) -> io::Result<u64>;
