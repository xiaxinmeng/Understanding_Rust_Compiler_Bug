rust
use std::borrow::Cow;

#[derive(Clone)]
struct Foo {
    foo: Cow<'static, [Foo]>,
}
