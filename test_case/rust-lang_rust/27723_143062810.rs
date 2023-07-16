 rust
unsafe impl<'a, T: Send> Send for Drain<'a, T> {}
unsafe impl<'a, T: Send> Sync for Drain<'a, T> {}
