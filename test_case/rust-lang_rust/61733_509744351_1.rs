rust
fn foo() -> u8 {
    { 0 } // <- trailing?

    fn bar() {}
}
