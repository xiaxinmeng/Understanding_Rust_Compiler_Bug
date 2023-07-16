
pub struct Foo<P>(P);
pub trait Bar { type Arthur: Bar; }
pub trait Baz<P: Bar> { type Arthur: Baz<P::Arthur>; }
impl<P: Bar> Bar for Foo<P> { type Arthur = P; }
pub trait BazFoo<P>: Baz<Foo<P>> where P: Bar { }
