rust
#![feature(generic_const_exprs)]

trait Foo {
    type Assoc;
}

trait Bar<const N: usize> {}

impl Foo for () {
    type Assoc = ();
}
impl<const N: usize> Bar<N> for () {}

trait Trait {
    const ASSOC: usize;
}

fn foo<T: Trait>() -> impl Foo<Assoc = impl Bar<{ T::ASSOC }>> {}
