
pub fn rename(old: &CString, new: &CString) -> IoResult<()> {
    let old = try!(to_utf16(old));
    let new = try!(to_utf16(new));
    super::mkerr_winbool(unsafe {
        libc::MoveFileExW(old.as_ptr(), new.as_ptr(),
        libc::MOVEFILE_REPLACE_EXISTING)
    })
}
