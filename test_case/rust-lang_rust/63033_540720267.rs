rust
struct Foo;

impl Foo {
    async fn wat(&self, _: &'static str, _: Bar<'_>) {}
}

struct Bar<'a>(Box<dyn std::fmt::Debug + 'a>);
