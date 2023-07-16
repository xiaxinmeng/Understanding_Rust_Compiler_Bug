 rust
struct Foo<I, T> where I: Iterator<Item=T> { iter: I }
