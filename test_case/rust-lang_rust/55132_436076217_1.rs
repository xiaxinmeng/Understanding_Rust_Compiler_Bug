rust
pub unsafe fn spawn_unchecked<F, T>(self, f: F) -> io::Result<JoinHandle<T>> where
	F: FnOnce() -> T, F: Send, T: Send 
{
	imp::Thread::new(..., Box::new(f))
}
