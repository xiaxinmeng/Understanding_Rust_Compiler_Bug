rust
let atomic = AtomicI32::new(1);
unsafe {
    let ptr = (&*(&atomic as *const AtomicI32 as *const UnsafeCell<i32>)).get();
    ffi(ptr);
}
