rust
pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>> where
	F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
	imp::Thread::new(..., Box::new(f))
}
