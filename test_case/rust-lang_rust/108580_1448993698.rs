rust
#![feature(return_position_impl_trait_in_trait)]

trait Foo {
    fn bar(&self) -> impl Iterator<Item = impl Sized> + '_;
}

impl Foo for () {
    fn bar(&self) -> impl Iterator + '_ {
        vec![()].into_iter()
    }
}
