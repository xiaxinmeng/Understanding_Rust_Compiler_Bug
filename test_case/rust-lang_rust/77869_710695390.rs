rust
macro_rules! cons {
    () => (
        ()
    );
    ($head:tt) => (
        ($head, ())
    );
    ($head:tt, $($tail:tt),*) => (
        ($head, cons!($($tail),*))
    );
}

fn blah() {
    let cons!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) = todo!();
}
