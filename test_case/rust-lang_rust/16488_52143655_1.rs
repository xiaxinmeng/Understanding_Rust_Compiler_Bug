 rust
impl rust_allocated_example {
    // XXX: This value never calls Drop
    pub fn new() -> rust_allocated_example {
        unsafe {
            let instance: rust_allocated_example = std::mem::uninitialized();
            use_example_constructor(&instance);
            instance
        }
    }
