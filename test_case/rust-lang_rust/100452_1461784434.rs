rust
pub async fn something(path: &[usize]) -> usize {
    // Without this async block it doesn't ICE
    async {
        match path {
            [] => 1,
            //[1] => 2,
            _ => 1,
        }
    }
    .await
}
