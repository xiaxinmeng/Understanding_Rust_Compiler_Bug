
extern crate libc;

use std::{env};
use std::ffi::CString;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let libtest_path = CString::new("/Users/john/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/libtest-bbaa058d367874e2.dylib").unwrap();
    let handle = unsafe {
        libc::dlopen(libtest_path.as_ptr(), libc::RTLD_NOW | libc::RTLD_GLOBAL)
    };

    if handle.is_null() {
        panic!("failed to open libtest");
    }

    assert!(false);
}
