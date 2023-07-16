rust
    fn decrement(&self, token: SignalToken) -> StartResult {
        unsafe {
/*253*/     assert_eq!(self.to_wake.load(Ordering::SeqCst), 0);
            let ptr = token.cast_to_usize();
            self.to_wake.store(ptr, Ordering::SeqCst);
