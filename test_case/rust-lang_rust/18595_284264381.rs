rust
fn foo<F: Fn() -> bool>(_: F) {}

fn main() {
        foo(|| { true; });
}
