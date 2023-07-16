 rust
#[derive(Debug)] struct Foo;
struct Bar;

#[derive(Debug)] struct Baz<T>(u64);

fn main() {
    println!("{:?}", Baz::<Foo>(10));
    println!("{:?}", Baz::<Bar>(10));  // Does not compile.
}
