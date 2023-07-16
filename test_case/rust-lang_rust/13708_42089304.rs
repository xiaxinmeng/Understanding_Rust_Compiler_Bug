 rust
fn log(record: &LogRecord) -> IoResult<()> {
    try!(write!(stdout, "{} {}: ", record.file, record.line));
    fmt::write(stdout, record.args)
}
