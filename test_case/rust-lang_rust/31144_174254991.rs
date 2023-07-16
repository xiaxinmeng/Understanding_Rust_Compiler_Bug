 rust
fn foo() {}

fn bar() {
    foo();
    use qux::foo;
    foo();
}
