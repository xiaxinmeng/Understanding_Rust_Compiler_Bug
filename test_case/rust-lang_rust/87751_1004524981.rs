rust
fn bar<R>(mut r: &R) where for<'r> &'r R: std::io::Read {
    let _ = std::io::Read::read_exact(&mut r, &mut []);
}
