rust
use std::ffi::{CString, NulError};

#[no_mangle]
pub extern "Rust" fn make_c_string()->Result<CString, NulError>{
    CString::new("Hello world!")
}

