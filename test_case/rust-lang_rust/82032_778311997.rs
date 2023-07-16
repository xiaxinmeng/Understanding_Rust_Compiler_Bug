rust
pub struct Handle {}
impl Handle {
    async fn flush(&mut self) {}
}

async fn die_horribly(thing: &[Handle]) {
    for v in thing {
        v.flush().await;
    }
}
