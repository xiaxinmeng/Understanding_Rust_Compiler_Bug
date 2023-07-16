
async fn a<F: Future<Output=()>, G: Future<Output=()>>(f: F, g: G) -> () {
    f.await;
    g.await;
}

fn b<F: Future<Output=()>, G: Future<Output=()>>(f: F, g: G) -> impl Future<Output = ()> {
    async move {
        f.await;
        g.await;
    }
}
