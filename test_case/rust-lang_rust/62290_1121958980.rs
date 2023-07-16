rust
async fn sayer() {
    let mut hs = Vec::new();
    for n in (0..5).rev() {
        hs.push(tokio::spawn(sleep_n(n)));
    }
    for h in hs {
        h.await.unwrap();
    }
}

