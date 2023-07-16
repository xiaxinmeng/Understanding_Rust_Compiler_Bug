rust
extern "x86-interrupt" fn handler(stack_frame: &ExceptionStackFrame) {
    use core::sync::atomic::{AtomicUsize, Ordering};
    static C: AtomicUsize = AtomicUsize::new(0);

    C.fetch_add(1, Ordering::Relaxed);
}
