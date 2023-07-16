 rust
mod foo {
    extern crate libc;
}

use foo::libc;

static mut BAR: *const libc::c_void = baz as *const libc::c_void;

fn baz() {}

fn main() {}
