rust
async fn test() {
    let mut x = None;
    loop {
        if let Some(x) = x.as_mut() {
            x.await;
        }
        x = Some(std::future::ready(1u32));
    }
}
