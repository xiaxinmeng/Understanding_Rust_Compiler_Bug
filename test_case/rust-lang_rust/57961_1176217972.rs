rust
#![feature(type_alias_impl_trait)]

type X = impl Sized;

trait Foo {
    type Bar: Iterator<Item = X>;
    
    fn into_bar(self) -> Self::Bar;
}

impl Foo for () {
    type Bar = std::vec::IntoIter<X>;
    
    fn into_bar(self) -> Self::Bar {
        vec![22, 44].into_iter()
    }
}

fn incoherent() {
    let f: X = 22_i32;
}

fn main() {}
