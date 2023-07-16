Rust
struct Foo;

impl Foo {
    fn method<T>(&self) {
        type X<This, T> = (This, T);
    }
}
 
fn main() { }
