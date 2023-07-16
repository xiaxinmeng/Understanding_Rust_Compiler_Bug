rust
use std::borrow::Cow;

#[derive(Clone)]
struct Foo<'a> {
    children: Cow<'a, [Self]>,
    _foo: (),
}
