rust
// Increment the strong reference count
let waker: Arc<W> = Arc::from_raw(waker as *const W);
mem::forget(Arc::clone(&waker));

// Decrement the strong reference count
mem::drop(Arc::from_raw(waker as *const W));
