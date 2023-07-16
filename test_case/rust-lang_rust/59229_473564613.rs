rust
fn from_reader(mut reader: impl BufRead) -> Result<CString, std::io::Error>
{
    let mut buffer = Vec::new();
    reader.read_until(0, &mut buffer)?;
    if buffer.len() > 0 {
        // 0 has been read into the vector, pop it
        buffer.pop(); //TODO: consider in-place transmute
    } else {
        // no bytes have been read, nothing to do
    }
    return Ok(unsafe { CString::from_vec_unchecked(buffer) });
}
