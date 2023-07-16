rust
mod First {
    trait Foo { fn m(self: Box<Self>); }
    fn foo<T: Foo>(a: T) {
        a.m();
    }
}
mod Second {
    trait Bar { fn m(self: std::sync::Arc<Self>); }
    fn bar(b: impl Bar) {
        b.m();
    }
}
