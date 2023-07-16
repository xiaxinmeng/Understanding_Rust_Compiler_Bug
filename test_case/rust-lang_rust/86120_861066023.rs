rs
pub struct Foo;

/// You should really try [`Self::bar`]
pub type Bar = Foo;

impl Bar {
    pub fn bar() {}
}
