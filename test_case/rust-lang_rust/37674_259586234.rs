 rust
fn catch_unwind<F, R>(f: F) -> Result<R> 
    where F: FnOnce() -> R + Send + 'static,
          R: Send + 'static,
{
    thread::spawn(f).join()
}
