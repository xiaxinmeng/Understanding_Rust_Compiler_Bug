rust
fn named_thread<F, T>(name: T, f: F) -> JoinHandle<()>
    where F: FnOnce(), F: Send + 'static, T: Into<String>
{
    thread::Builder::new()
        .name(name.into())
        .spawn(f)
        .unwrap()
}
