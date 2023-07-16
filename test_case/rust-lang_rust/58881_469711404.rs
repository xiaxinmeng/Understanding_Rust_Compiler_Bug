rust
use std::os::raw::c_char;

pub type cubeb_log_callback = Option<unsafe extern "C" fn(*const c_char, ...)>;
extern "C" {
    pub static g_cubeb_log_callback: cubeb_log_callback;
}

pub fn foo(index: u32) {
    unsafe {
        let op = "Adding";
        let dev = "sink";
        if let Some(log_callback) = g_cubeb_log_callback {
            let cstr = ::std::ffi::CString::new("%s:%d: {} {} index {}\n").unwrap();
            log_callback(cstr.as_ptr(), file!(), line!(), op, dev, index);
        }
    }
}

fn main() {
    let args = ::std::env::args().count();
    foo(args as u32);
}
