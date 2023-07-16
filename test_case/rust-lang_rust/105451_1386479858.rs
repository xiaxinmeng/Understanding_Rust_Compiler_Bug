 rust
#![feature(impl_trait_in_fn_trait_return)]
fn curry<P1, P2, R>(f: impl FnOnce(P1, P2) -> R) -> impl FnOnce(P1) -> impl FnOnce(P2) -> R {
    |p1| |p2| f(p1, p2)
}
