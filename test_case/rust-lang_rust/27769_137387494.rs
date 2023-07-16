 rust
#[repr(i8)] enum RustAllocChar {...}

impl CString {
    fn into_raw(self) -> *const RustAllocChar;
    unsafe fn from_raw(ptr: *const RustAllocChar) -> CString;
}
