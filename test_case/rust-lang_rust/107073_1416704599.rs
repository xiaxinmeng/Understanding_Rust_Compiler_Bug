rust
#![feature(type_alias_impl_trait)]
type Alias = impl Default;

pub fn foo() {
    #[derive(Default)] struct Foo;
    fn _f() -> Alias { Foo }
}

pub fn bar() -> Alias {
    Alias::default()
}
