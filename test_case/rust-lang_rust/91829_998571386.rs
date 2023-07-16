rust
use std::ops::Not;

fn compose_mut<A, B, C>(
    mut f: impl FnMut(A) -> B,
    mut g: impl FnMut(B) -> C,
) -> impl FnMut(A) -> C {
    move |x| g(f(x))
}

fn predicate(n: &usize) -> bool {
    *n == 0
}

fn main() {
    let b = compose_mut(predicate, bool::not);
}
