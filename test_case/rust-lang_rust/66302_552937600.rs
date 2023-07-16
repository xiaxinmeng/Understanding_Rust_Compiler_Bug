rust
    const fn foo() -> usize {
        static FOO: AtomicUsize = AtomicUsize::new(0);
        FOO.fetch_add(1, Ordering::Relaxed)
    }
    