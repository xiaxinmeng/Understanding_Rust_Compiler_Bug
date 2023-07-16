 rust
fn foo() { }

fn bar() -> &'static fn() {
    &'static foo
}
