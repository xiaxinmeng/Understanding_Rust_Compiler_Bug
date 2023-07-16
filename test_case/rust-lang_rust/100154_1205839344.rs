rust
fn foo(i: impl std::fmt::Display) {}

fn main() {
    foo::<()>(());
}
