rust
/// # Safety
///
///   - `BufRead` implementor mut enforce its contract
unsafe
fn from_reader_unchecked (
    mut reader: impl BufRead,
) -> Result<CString, ::std::io::Error>
{
    let mut buffer = Vec::new();
    reader.read_until(b'\0', &mut buffer)?;
    if let Some(&b'\0') = buffer.last() {
        // # Safety
        //
        //   - no null bytes before `buffer.last()` (thanks to `.read_until()` contract)
        //
        //   - last byte has already been checked to be null
        CString::from_vec_unchecked(buffer)
    } else {
        buffer.reserve_exact(1);
        buffer.push(b'\0');
        // # Safety
        //
        //   - no null bytes before `buffer.last()` (thanks to `.read_until()` contract)
        //
        //   - terminating null byte has been appended
        CString::from_vec_unchecked(buffer)
    }
}

/// # Safety:
///
///   - `read_until` must enforce its contract.
unsafe trait TrustedBufRead : BufRead {}

#[inline]
fn from_reader (
    reader: impl TrustedBufRead,
) -> Result<CString, ::std::io::Error>
{
    unsafe { from_reader_unchecked(reader) }
}
