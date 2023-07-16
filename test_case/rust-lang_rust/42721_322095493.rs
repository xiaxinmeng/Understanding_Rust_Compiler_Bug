rust
#![feature(optin_builtin_traits)]

trait NotEq {}
impl NotEq for .. {}
impl<X> !NotEq for (X, X) {}

struct X<A>(A);
impl<A, B> From<X<B>> for X<A>
    where
        A: From<B>,
        (A, B): NotEq,
{
    fn from(a: X<B>) -> Self { X(a.0.into()) }
}

fn main() {}
