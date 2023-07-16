rust
struct Foo<T> {}

impl<T> Foo<T> {
    thread_local! {
        static TL: T = {};
    }
}
