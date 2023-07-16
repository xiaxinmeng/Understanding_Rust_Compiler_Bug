 rust
fn foo<F: Foo>() {}

fn main() {
    foo::<Vec<i32>>();
}
