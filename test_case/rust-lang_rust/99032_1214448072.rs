rust
    struct PanicPayload(ManuallyDrop<dyn Any>);
    impl Drop for PanicPayload {
        fn drop(&mut self) {
            panic_count::increment();
            if let Err(_err) = catch_unwind(|| unsafe { ManuallyDrop::drop(&mut self.0) }) {
                // should have aborted already
                rtprintpanic!("fatal error: unwound while dropping panic payload");
                rtabort!();
            }
            panic_count::decrement();
        }
    }
    impl Any for PanicPayload {
        // questionable; what this PR is doing
        fn type_id(&self) -> TypeId { self.0.type_id() }
    }