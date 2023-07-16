
impl AtomicT { 
    // UB if pointer is not appropriately aligned or does not satisfy any other machine-specific requirements. Doing this with `T = atomic_T_t` should always be valid.
    unsafe fn from_ptr(x: *mut T) -> &AtomicT { ... }
}
