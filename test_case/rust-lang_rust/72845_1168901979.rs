rust
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![allow(incomplete_features)]

//--------------------------------------------------

trait Depth {
    const C: usize;
}

trait Type {
    type At: Depth;
}

//--------------------------------------------------

enum Predicate<const B: bool> {}

trait Satisfied {}

impl Satisfied for Predicate<true> {}

//--------------------------------------------------

trait Spec1 {}

impl<T: Type> Spec1 for T where Predicate<{ T::At::C > 0 }>: Satisfied {}

trait Spec2: Spec1 {}

impl<T: Type + Spec1> Spec2 for T where Predicate<{ T::At::C > 1 }>: Satisfied {}

//--------------------------------------------------

trait Foo {
    fn Bar();
}

impl<T: Spec1> Foo for T {
    default fn Bar() {}
}

impl<T: Spec2> Foo for T {
    fn Bar() {}
}
