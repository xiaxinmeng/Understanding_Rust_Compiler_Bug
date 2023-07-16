rust
loop {
    let buf = reader.fill_buf()?;
    if buf.is_empty() { break; }
    let consumed = file_out.write(buf)?;
    drop(buf);
    reader.consume(consumed);
}
