rust
// in the "Header" type, which is a private type in maitake
pub(crate) const fn new_stub() -> Self {
    unsafe fn nop(_ptr: TaskRef) -> Poll<()> {
        #[cfg(debug_assertions)]
        unreachable!("stub task ({_ptr:?}) should never be polled!");
        #[cfg(not(debug_assertions))]
        Poll::Pending
    }

    unsafe fn nop_deallocate(ptr: NonNull<Header>) {
        unreachable!("stub task ({ptr:p}) should never be deallocated!");
    }

    Self {
        run_queue: mpsc_queue::Links::new_stub(),
        state: StateCell::new(),
        vtable: &Vtable {
            poll: nop,
            deallocate: nop_deallocate,
        },
    }
}
