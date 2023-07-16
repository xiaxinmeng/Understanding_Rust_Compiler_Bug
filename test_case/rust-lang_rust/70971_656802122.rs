
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

trait M {
    type T;
    fn get(&self) -> Self::T;
}

impl M for u8 {
    type T = impl Iterator<Item = u8>;
    fn get(&self) -> Self::T {
        0..8
    }
}

fn main() {
    let _x: &dyn M<T = impl Iterator<Item = u8>> = &2;
}
