rust
#![feature(c_variadic)]
use std::ffi::VaList;

extern "C" {
    // vprintf signature is `int vprintf(const char * format, va_list arg)`
    #[no_mangle]
    fn vprintf(_: *const libc::c_char, _: VaList) -> libc::c_int;
}

#[no_mangle]
unsafe extern "C" fn variadic(fmt: *const libc::c_char, a: ...) -> libc::c_int {
    // `va_start` in theory should be automatically called giving you a
    // `VaList` at `a` here.
    vprintf(fmt, a)
    // `va_end` should be called here.
}

fn main() {
    let fmt = std::ffi::CString::new("test %d %d %d\n").expect("new failed");
    unsafe {
        variadic(fmt.as_ptr(), 1, 2, 3);
    }
}
