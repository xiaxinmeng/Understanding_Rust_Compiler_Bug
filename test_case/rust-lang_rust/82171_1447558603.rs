rust
#![feature(type_alias_impl_trait)]

mod mod_foo {
    pub type ReturnTy = impl Iterator<Item = String> + 'static;
    pub fn foo<T>(a: T) -> ReturnTy
    where
        T: std::fmt::Display,
    {
        std::iter::once(a.to_string())
    }
}

fn bar(a: &str) -> impl Iterator<Item = String> + 'static {
    mod_foo::foo(a)
}
