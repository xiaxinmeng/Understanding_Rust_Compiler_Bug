rust
#![feature(trait_alias)]

struct X;

fn bar() -> impl Baz<X, X> {
    |x: &X| x
}

trait Baz<A, B> = Fn(&A) -> &B;
