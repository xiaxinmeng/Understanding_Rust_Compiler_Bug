 Rust
// lib.rs
#![crate_type = "dylib"]
#![crate_id = "r"]

#[link(name = "ext", kind = "static")]
extern "C" {
    pub fn func5();
}

#[no_mangle] pub fn func1() {} // exported
#[no_mangle] fn func2() {}
fn func3() { unsafe { func5(); } } // use dllexported symbol
