rust
pub fn drop_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Option<R> {
    match catch_unwind(f) {
        Ok(r) => Some(r),
        Err(e) => {
            // Dropping the panic payload could panic, so prevent that.
            // Treat the thread as still panicking.
            crate::panicking::panic_count::increase();
            let e = AssertUnwindSafe(e);
            // Mark the drop closure as rustc_allocator_nounwind. This tells
            // rustc to emit its own shim to turn unwinds into aborts.
            match catch_unwind(
                #[rustc_allocator_nounwind]
                || drop(e),
            ) {
                Ok(()) => {}
                Err(_dont_drop) => {
                    // We still unwound again somehow; just abort directly.
                    rtprintpanic!("thread panicked while dropping panic payload. aborting.\n");
                    crate::sys::abort_internal();
                }
            }
            // Dropping did not panic.
            crate::panicking::panic_count::decrease();
            None
        }
    }
}
