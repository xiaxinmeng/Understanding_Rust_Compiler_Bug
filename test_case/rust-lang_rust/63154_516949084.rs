rust
#![allow(dead_code)]

trait HasAssocType {
    type Inner;
}

impl HasAssocType for () {
    type Inner = ();
}

trait Tr<I, T>
where
    Self: Fn(I),
{
}

impl<I, T, Q> Tr<I, T> for Q where Q: Fn(I) {}

fn f<T>() -> impl Tr<T, T::Inner>
where
    T: HasAssocType,
{
    |_| ()
}

fn g<T, Y, F>(f: F) -> impl Tr<T, Y>
where
    F: Tr<T, Y>,
{
    f
}

fn h() {
    g(f())(());
}
