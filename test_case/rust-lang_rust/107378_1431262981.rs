rust
#![feature(anonymous_lifetime_in_impl_trait, type_alias_impl_trait)]

type Foo<'a> = impl IntoIterator<Item = &'a i32>;

// TAIT won't work without this function
pub fn type_resolution<'a>(slice: &'a [i32]) -> Foo<'a> {
    slice
}

pub fn bar<'a>(slice: &'a [i32]) {
    foo([type_resolution(slice)]);
}

// Implies foo<'a>(_: impl IntoIterator<Item = Foo<'a>) {}
pub fn foo(_: impl IntoIterator<Item = Foo>) {}
