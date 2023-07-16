rust
fn a_desugared<F: Future<Output=()>, G: Future<Output=()>>(f: F, g: G) -> impl Future<Output = ()> {
    async move {
        let f = f;
        let g = g;
        f.await;
        g.await;
    }
}
