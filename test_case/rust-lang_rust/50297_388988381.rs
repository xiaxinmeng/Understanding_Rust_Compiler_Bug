rust
/// IO sink that uses the `print` macro, for test stdout capture.
pub struct PrintWriter;
impl std::io::Write for PrintWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        print!("{}", String::from_utf8_lossy(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { std::io::stdout().flush() }
}

/// IO sink that uses the `eprint` macro, for test stdout capture.
pub struct EPrintWriter;
impl std::io::Write for EPrintWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        eprint!("{}", String::from_utf8_lossy(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { std::io::stdout().flush() }
}
