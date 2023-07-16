 rust
#![crate_type="lib"]

pub struct Foo;
pub mod traits {
    impl PartialEq for super::Foo {
        fn eq(&self, _: &super::Foo) -> bool { true }
    }
}
