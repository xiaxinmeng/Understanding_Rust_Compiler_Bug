rust
#![feature(type_alias_impl_trait)]
#![feature(const_generics)]

type Foo<const X: usize, const Y: usize> = impl Sized;

fn foo<const N: usize>(x: [u8; N]) -> Foo<N, N> {
    x
}
