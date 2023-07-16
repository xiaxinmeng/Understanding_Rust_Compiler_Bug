 rust
extern crate core;
extern crate libc;

use libc::c_int;
use core::ptr::Unique;

fn test_send<T:Send>(s: T) {}

fn main() {

    let x = Unique(0 as *mut libc::c_int);
    test_send(x);
    // test_send(0 as *mut libc::c_int); // this fails
}
