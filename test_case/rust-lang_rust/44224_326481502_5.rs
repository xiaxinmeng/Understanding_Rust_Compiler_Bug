rust
unsafe impl<'a, A, B> JSTraceable for fn(&A) -> B {
    #[inline]
    unsafe fn trace(&self, _: *mut JSTracer) {
        // Do nothing
    }
}
