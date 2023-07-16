rs
async fn recur_never() {
    if false {
        recur_never().await;
    }
}
