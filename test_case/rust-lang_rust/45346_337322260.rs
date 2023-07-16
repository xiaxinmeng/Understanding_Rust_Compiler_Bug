rust
fn main() {
    struct Foo(String);
    let _ = |&Foo(ref name)| {
        name.len()
    };
}
