rust
#![crate_type = "lib"]
#![allow(incomplete_features)]
#![feature(unsized_locals, unsized_fn_params)]

trait Unsized {
    fn foo(&self);
}

#[inline(never)]
#[no_mangle]
fn take_unsized_arg(arg: dyn Unsized) {
    arg.foo();
}

#[no_mangle]
fn use_unsized_arg(arg: dyn Unsized) {
    take_unsized_arg(arg);
}
