rust
    let my_allocator = unsafe { Locked::new(Heap::new(0xc00000 as *const usize, false)) };
    let a = Box::new_in(42usize, my_allocator);
