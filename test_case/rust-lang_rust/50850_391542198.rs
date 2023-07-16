rust
trait Future { }

struct GenFuture<G, R> {
    gen: G,
    marker: PhantomData<R>,
}

impl<G: Generator<Yield = ()>> Future for GenFuture<G, G::Return> { }

fn gen_future<G: Generator>(gen: G) -> GenFuture<G, G::Return> { /* ... */ }
