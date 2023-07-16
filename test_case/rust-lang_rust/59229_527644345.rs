rust
unsafe
fn from_reader_unchecked (
    mut reader: impl BufRead,
) -> Result<CString, ::std::io::Error>
{
    let mut buffer = Vec::new();
    reader.read_until(b'\0', &mut buffer)?;
    if let Some(&b'\0') = buffer.last() {
        buffer.pop()
        // # Safety
        // last was the first null byte encountered, it's been removed.
        CString::from_vec_unchecked(buffer)
    } else {
        // # Safety
        //  no null bytes before `buffer.last()` (thanks to `.read_until()` contract)
        CString::from_vec_unchecked(buffer)
    }
}
