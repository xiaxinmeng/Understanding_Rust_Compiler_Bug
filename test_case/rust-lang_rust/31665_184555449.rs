
extern crate libc;
extern crate time;

use libc::getaddrinfo;
use std::ffi::CString;
use std::ptr;

#[test]
fn it_works() {
    let count = 10000;

    let start = time::now();
    for _ in 0..count {
        call_getaddrinfo("google.com".to_string());
    }
    let end = time::now();

    let diff: time::Duration = end - start;

    println!("End ({0}) - Start ({1}) = {2} / {3} = {4}",
             end.ctime(),
             start.ctime(),
             diff,
             count,
             diff / count);
}

fn call_getaddrinfo(name: String) {
    // Function Signature:
    // pub unsafe extern fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int

    let hostname = CString::new(name).unwrap();

    unsafe {
        // let hints_ptr = &hints;
        let mut res = ptr::null_mut();
        let r = getaddrinfo(hostname.as_ptr(), ptr::null(), ptr::null(), &mut res);
        if r < 0 {
            panic!("Error from getaddrinfo");
        }
    };
}
