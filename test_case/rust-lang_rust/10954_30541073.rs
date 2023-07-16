
#[allow(dead_code)];
use std::libc::c_void;
use std::cast::transmute;

extern "C" fn a<T>(a: *c_void) {
    unsafe {
        let _: *A<T> = transmute(a);
    }
}

struct A<'a, T> {
    a: &'a T,
}

fn main() { }


task 'rustc' failed at 'assertion failed: !ty::type_has_params(t)', /build/rust-git/src/rust/src/librustc/middle/trans/common.rs:1117
error: internal compiler error: unexpected failure
This message reflects a bug in the Rust compiler. 
We would appreciate a bug report: https://github.com/mozilla/rust/wiki/HOWTO-submit-a-Rust-bug-report
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1 to get further details and report the results to github.com/mozilla/rust/issues
task '<main>' failed at 'explicit failure', /build/rust-git/src/rust/src/librustc/lib.rs:393
