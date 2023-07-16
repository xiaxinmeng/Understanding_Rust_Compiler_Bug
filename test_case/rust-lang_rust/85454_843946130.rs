rust
pub trait A {
    type Tar;
}

pub struct Foo {}

impl Foo {
    pub fn foo() -> <Self as A>::Tar {}
}

impl A for Foo { type Tar = u32; }
