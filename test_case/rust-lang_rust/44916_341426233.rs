rust
// keep these:
impl<'a> TryFrom<&'a [u8]> for &'a CStr { … }
impl TryFrom<Vec<u8>> for CString { … }
impl TryFrom<CString> for String { … }

// and remove the rest?
