 rust
fn escape_char(writer: &mut io::Writer, v: char) -> Result<(), io::IoError> {
    let mut buf = [0, .. 4];
    let sz = v.encode_utf8(&mut buf).unwrap();
    escape_bytes(writer, buf[mut ..sz])
}
