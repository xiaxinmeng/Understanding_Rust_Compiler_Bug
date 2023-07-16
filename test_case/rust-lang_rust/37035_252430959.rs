 rust
struct Foo<A> { inner: A };

trait Bar { fn bar(); }

impl Bar for Foo<i32> {
    fn bar() {
        Self { inner: 1.5f32 };
    }
}
