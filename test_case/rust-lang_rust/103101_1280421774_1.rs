rust
fn panic_guard<T>(block: impl FnOnce() -> T, panic_handler: impl FnOnce()) -> T {
    struct PanicGuard<PanicHandler: FnOnce()>(PanicHandler);

    impl<P: FnOnce()> Drop for PanicGuard<P> {
        fn drop(&mut self) {
            self.0();
        }
    }

    let guard = PanicGuard(panic_handler);
    let result = block();
    std::mem::forget(guard);
    result
}
