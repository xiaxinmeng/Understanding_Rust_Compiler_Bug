rust
#[repr(transparent)]
pub struct CStr(str);

impl CStr {
    /// Creates a new CStr from a str without performing any additional checks. `data` _must_ end
    /// with a NUL byte, and should only have only a single NUL byte, or the string will be
    /// truncated.
    pub const unsafe fn new_unchecked(data: &str) -> &CStr {
        &*(data as *const str as *const CStr)
    }
}
