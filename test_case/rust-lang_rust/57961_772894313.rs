
#![feature(type_alias_impl_trait)]

type X = impl Sized;

trait Foo {
    type Bar: Iterator<Item = Self::_0>;
    type _0;
}

impl Foo for () {
    type Bar = std::vec::IntoIter<u32>;
    type _0 = X;    
}

fn main() {}
