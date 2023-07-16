rust
extern "C" fn foo() {
    panic!("uh oh");
}

#[test]
fn does_it_break() {
    foo()
}
