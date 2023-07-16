rust
async fn add(x: u32, y: u32) -> u32 {
    let a = async { x + y };
    a.await
}
