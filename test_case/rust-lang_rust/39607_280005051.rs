
pub trait Foo<B>: {type B: Foo<B>;}
impl<A> Foo<Self::B> for A   { type B = A; }
