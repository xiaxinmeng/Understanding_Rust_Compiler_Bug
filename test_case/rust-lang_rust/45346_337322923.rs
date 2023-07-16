rust
fn main() {
    struct Foo(String);
    let _ = |&Foo(ref name): &Foo| {
        name.len()
    };
}
