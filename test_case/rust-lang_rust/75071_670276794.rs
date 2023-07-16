rust
pub fn replace_with<T>(val: &mut T, f: impl FnOnce(T)->T) {
    struct PanicGuard;
    impl Drop for PanicGuard {
        fn drop(&mut self) {
            // replace_with is not safe when f panics, so use a guard to
            // trigger a double panic which will abort without drop.
            //
            // This could also be std::intrinsics::abort. 
            panic!("closure passed to replace_with must not panick");
        }
    }
    let guard = PanicGuard;
    unsafe {
        let value = ptr::read(val);
        let value = f(value);
        ptr::write(val, value);
    }
    mem::forget(guard);
}
