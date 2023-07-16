
#![feature(never_type, specialization)]

pub struct Foo<P>(!, P);
pub trait Bar { type Arthur; }
pub trait Baz<P> { type Arthur: Baz<<P as Bar>::Arthur>; }
impl<P> Bar for P { default type Arthur = !; }
impl<P: Bar> Bar for Foo<P> { type Arthur = P; }
pub trait BazFoo<P>: Baz<Foo<P>> { }
