rust
fn bar_ret_2<const N: usize, const M: usize>(a: Foo<N>, b: Foo<M>) -> Foo<{ max(N, M) }> {
    bar_ret(a, b)
}
