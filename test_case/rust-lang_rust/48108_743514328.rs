rust
unsafe fn increment_count<T>(ptr: *const T) {
    let rc = Rc::from_raw(ptr);
    std::mem::forget(rc.clone());
    std::mem::forget(rc);
}
