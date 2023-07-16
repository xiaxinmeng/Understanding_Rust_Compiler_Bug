rust
#[allow(unconditional_recursion)]
fn bar() {
    bar()
}

#[no_mangle]
pub extern "C" fn foo() {
    bar()
}
