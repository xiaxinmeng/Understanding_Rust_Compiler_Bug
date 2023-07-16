rust
fn foo<'a: 'b, 'b: 'a>() {}
fn main() {
    foo::<'static>();
}
