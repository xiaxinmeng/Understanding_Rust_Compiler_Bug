rust
impl<T: RefUnwindSafe, F: UnwindSafe> RefUnwindSafe for SyncLazy<T, F> {}
