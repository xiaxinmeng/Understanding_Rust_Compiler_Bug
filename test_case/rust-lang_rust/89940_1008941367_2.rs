rust
use std::borrow::Cow;

struct Foo<'a> {
    children: Cow<'a, [Self]>,
}

impl<'a> Clone for Foo<'a> {
    fn clone(&self) -> Self {
        let children = match self.children {
            Cow::Owned(x) => Cow::Owned(x.clone()),
            Cow::Borrowed(x) => Cow::Borrowed(x),
        };
        Self { children }
    }
}
