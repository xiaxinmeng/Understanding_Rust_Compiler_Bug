 rust
fn escape_char(writer: &mut io::Writer, v: char) -> Result<(), io::IoError> {
    let mut buf = [0, .. 4];
    v.encode_utf8(&mut buf);
    escape_bytes(writer, &mut buf)
}
