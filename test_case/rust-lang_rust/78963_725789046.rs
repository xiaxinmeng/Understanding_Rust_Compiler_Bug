rust
// The interner is pointed to by a thread local value which is only set on the main thread
// with parallelization is disabled. So we don't allow `Span` to transfer between threads
// to avoid panics and other errors, even though it would be memory safe to do so.
#[cfg(not(parallel_compiler))]
impl !Send for Span {}
#[cfg(not(parallel_compiler))]
impl !Sync for Span {}
