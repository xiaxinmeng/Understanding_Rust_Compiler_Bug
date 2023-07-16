rust
#![feature(tool_attributes, decl_macro)]

mod rustfmt {
    pub macro zzz() {}
}

#[rustfmt::zzz] // Should be: ERROR, expected attribute macro, found fn-like macro
fn f() {}

fn main() {}
