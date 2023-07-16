rust
struct Fib<T: Expr<i32>>(PhantomData<T>);

impl<N: Expr<i32>> Expr<i32> for Fib<N> {
    const VALUE: i32 = If::<
        Eq<I32<0>, N>,
        I32<1>,
        If<Eq<I32<1>, N>, I32<1>, Add<Fib<Sub<N, I32<1>>>, Fib<Sub<N, I32<2>>>>>,
    >::VALUE;
}
