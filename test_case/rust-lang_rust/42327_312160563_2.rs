rust
fn async_stuff() -> impl Future<V, E> {
    let t = fetch_t();
    t.and_then(|t_val| {
        let u: Result<U, E> = calc(t_val);
        async_2(u?)
    })
}
fn fetch_t() -> impl Future<T, E> {}
fn calc(t: T) -> Result<U,E> {}
