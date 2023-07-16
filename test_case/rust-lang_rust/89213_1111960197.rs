rust
async fn wait(a: u32) {
    std::future::ready(()).await;
    drop(a);
}
