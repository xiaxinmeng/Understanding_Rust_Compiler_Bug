rust
async fn a<F: Future<Output=()>, G: Future<Output=()>>(f: F, g: G) -> () {
    f.await;
    g.await;
}
