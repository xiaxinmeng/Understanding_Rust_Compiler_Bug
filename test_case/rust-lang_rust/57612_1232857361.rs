rust
#![feature(trait_alias)]
#![feature(closure_lifetime_binder)]

struct X;

fn bar() -> impl Baz<X, X> {
    for<'a> |x: &'a X| -> &'a X { x }
}

trait Baz<A, B> = Fn(&A) -> &B;
