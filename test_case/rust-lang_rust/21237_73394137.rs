 rust
#[derive(Debug)] // T: Debug is not inserted
struct Foo<T>;

#[derive(Debug)] // T: Debug is inserted, but T is still phantom
struct Bar<T> {
    x: Foo<T>
}
