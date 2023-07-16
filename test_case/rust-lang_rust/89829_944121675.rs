rust
#![feature(generic_const_exprs)]
struct Foo<'a>([&'a u32; std::mem::size_of::<&'a u32>()]);
fn covariant<'a>(v: &'a Foo<'static>) -> &'a Foo<'a> {
    v
}
