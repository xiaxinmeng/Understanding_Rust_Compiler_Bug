rust
let value = UnsafeCell::new(0);
let ready = AtomicBool::new(false);

// thread 1:
unsafe { *value.get() = 123 };
ready.store(true, Release);

// thread 2:
if ready.load(Acquire) {
    dbg!(unsafe { *value.get() });
}
