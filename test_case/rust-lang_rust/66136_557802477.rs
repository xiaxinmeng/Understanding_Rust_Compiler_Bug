rust
let atomic = AtomicI32::new(1);
let ptr = &atomic as *const AtomicI32 as *mut i32;
unsafe {
    ffi(ptr);
}
