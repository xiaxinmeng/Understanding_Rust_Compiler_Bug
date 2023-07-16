 rust
#![crate_type="lib"]

pub trait Bar {}

#[doc(hidden)]
pub mod hidden {
    pub struct Foo;
}

// impl shown in `foo::Bar`s implementors section (but without link)
impl Bar for hidden::Foo {}
