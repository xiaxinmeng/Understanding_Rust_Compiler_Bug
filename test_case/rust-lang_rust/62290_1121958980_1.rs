rust
async fn sayer() {
    let mut hs = Vec::new();
    for n in (0..5).rev() {
        hs.push(tokio::spawn(sleep_n(n)));
    }
    let _ = hs.into_iter().map(async |i|  i.await.unwrap());
}
