rust
#![feature(used_with_arg)]

#[used(compiler)]
pub static _FOO: unsafe extern "C" fn() -> () = foo;

#[no_mangle]
pub unsafe extern "C" fn foo() {}
