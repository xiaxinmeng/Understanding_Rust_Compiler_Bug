rust
struct MyFoo<F> {
      f: F,
      _pinned: PhantomPinned,
}

impl<F> Drop for MyFoo<F> {
    fn drop(&mut self) {
        // *oops*, I have unpinned access to what might be a pinned instance
    }
}
