Rust
#![feature(core_intrinsics, rustc_attrs)]

use std::intrinsics::rustc_peek;

fn k() {}

#[rustc_mir(rustc_peek_definite_init,stop_after_dataflow)]
fn foo() {
    rustc_peek(k());
}

fn main() {}
