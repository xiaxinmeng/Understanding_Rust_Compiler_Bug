 rust
struct Foo<F, T> where F: Fn() -> T {
    f: F,
}
