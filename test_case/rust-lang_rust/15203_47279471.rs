 rust
    let mut sieve = Vec::with_capacity(size);
    unsafe {
        let p = sieve.as_mut_ptr();
        std::ptr::set_memory(p, 1, size);
        sieve.set_len(size);
    }
