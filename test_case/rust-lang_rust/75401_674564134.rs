rust
macro_rules! lit_cstr {
    ($s:literal) {
        (concat!($s, "\0").as_bytes().as_ptr() as *const ::libc::c_char)
    };
}
