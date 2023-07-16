
fn from_reader(mut reader: impl BufRead) -> Result<CString, std::io::Error>
{
    let mut buffer = Vec::new();
    reader.read_until(0, &mut buffer)?;
    if buffer.len() > 0 {
        return Ok(CString { inner: buffer.into_boxed_slice() });
    } else {
        return Ok(unsafe { CString::from_vec_unchecked(buffer) });
    }
}
