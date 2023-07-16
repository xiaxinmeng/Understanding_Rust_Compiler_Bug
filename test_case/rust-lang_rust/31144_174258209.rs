 rust
fn foo() {}
fn bar() {
    foo(); // Prints "foo!"
    fn foo() { println!("foo!"); }
    foo(); // Prints "foo!"
}
