rust
#[derive(Copy, Clone)]
struct Foo<T>(T);
fn main() {
    [Foo(String::new()); 4];
}
