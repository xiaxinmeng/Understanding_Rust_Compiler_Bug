rust
fn foo2() {}

#[no_mangle]
pub extern fn bar() {
    foo2();
}
