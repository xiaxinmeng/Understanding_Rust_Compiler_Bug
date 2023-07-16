rust
// test.rs

#![crate_type = "cdylib"]

#[no_mangle]
pub fn foo() {
    panic!("test");
}
