Rust
    counter = Mutex::new(0);
    counter_ref = &counter;
    lock = Mutex::lock(counter_ref);
    if let Ok(_) = lock {} else {}
// end of scope of function
    drop(counter); // this frees the mutex
// end of scope of lock
    drop(lock); // this tries to unlock a lock after it was freed - UB!
