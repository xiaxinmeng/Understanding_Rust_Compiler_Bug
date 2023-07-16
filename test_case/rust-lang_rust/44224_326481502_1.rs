rust
/// A trait to allow tracing (only) DOM objects.
pub unsafe trait JSTraceable {
    /// Trace `self`.
    unsafe fn trace(&self, trc: *mut JSTracer);
}

unsafe impl<A, B> JSTraceable for fn(A) -> B {
    #[inline]
    unsafe fn trace(&self, _: *mut JSTracer) {
        // Do nothing
    }
}
