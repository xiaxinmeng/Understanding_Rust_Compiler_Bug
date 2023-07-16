rust
fn spawn<T: Send>(_: T) {}

async fn fetch(_: &()) {
    fn f<T>() {}
    spawn(f);
}

pub async fn spawn_fetch() {
    spawn(fetch(&()));
}
