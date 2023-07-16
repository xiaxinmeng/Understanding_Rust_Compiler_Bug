 rust
fn foo<F>(_: F) where F: Fn() {}

fn bar() {}

fn main() {
    foo(bar)
}
