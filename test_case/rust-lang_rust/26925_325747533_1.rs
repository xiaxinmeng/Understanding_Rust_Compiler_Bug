rust
struct Foo<T: Iterator> {
    data: T::Item
}
