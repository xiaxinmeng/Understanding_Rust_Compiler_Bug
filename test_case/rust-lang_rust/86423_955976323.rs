rust
fn fill_buf2(&mut self) -> Result<Option<&[u8]>> {
    let buf = self.fill_buf()?;
    if buf.is_empty() {
        None
    } else {
        Some(buf)
    }
}
