 rust
struct Foo<T> {
    one: Bar<T>,
    two: Bar<T>
}
struct Bar<T> {
    foo: Foo<T>
}
