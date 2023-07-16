
pub trait Foo<T> { }
pub trait Bar: Foo<<Self as Bar>::Quux> { type Quux; }
pub trait Arthur { type Wibble: Bar<Quux=()>; }
pub fn assert_is_foo<A: Foo<()>>() { }
pub fn assert_is_bar<B: Bar<Quux=()>>() { }

pub fn test<A: Arthur>() {
    let _: <A::Wibble as Bar>::Quux = ();
    assert_is_bar::<A::Wibble>();
    assert_is_foo::<A::Wibble>(); // the trait bound `<A as Arthur>::Wibble: Foo<()>` is not satisfied
}
