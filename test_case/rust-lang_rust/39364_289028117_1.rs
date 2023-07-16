rust
impl<T> Drop for Packet<T> {
    fn drop(&mut self) {
        // Note that this load is not only an assert for correctness about
        // disconnection, but also a proper fence before the read of
        // `to_wake`, so this assert cannot be removed with also removing
        // the `to_wake` assert.
        assert_eq!(self.cnt.load(Ordering::SeqCst), DISCONNECTED);
/*503*/ assert_eq!(self.to_wake.load(Ordering::SeqCst), 0); 
        assert_eq!(self.channels.load(Ordering::SeqCst), 0);
    }
}
