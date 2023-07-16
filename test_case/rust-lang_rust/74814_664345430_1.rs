rust
impl<T: RefUnwindSafe + ?Sized> UnwindSafe for &T {} // and Rc<T> and Arc<T> etc

fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> { .. }
