Rust
trait Uninhabitable { fn never(self) -> !; }
impl Uninhabitable for ! { fn never(self) -> !; }

struct MyType {}

// ok
impl<T> From<T> for MyType where T: Uninhabitable {
    fn from(t: T) -> Self { t.never() }
}
