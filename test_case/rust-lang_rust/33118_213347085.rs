 rust
fn bar() {
    let x = foo; // resolves to the inner fn `foo`
    let foo = 22;
    let y = foo; // resolves to the variable
    fn foo() { }
}
