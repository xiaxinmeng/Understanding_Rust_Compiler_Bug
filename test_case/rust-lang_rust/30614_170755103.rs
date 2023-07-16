 rust
macro_rules! cstr {
    ($s:expr) => {
        unsafe { ::std::ffi::CStr::from_bytes_unchecked(concat!($s, "\0").as_bytes()) }
    }
}
