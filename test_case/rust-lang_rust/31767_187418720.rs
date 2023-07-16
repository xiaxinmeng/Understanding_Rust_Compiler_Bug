 rust
// Load the initial value
let mut val = ATOMIC.load(Ordering::Relaxed);
loop {
    // Do something with it and determine the new value
    let new = do_something(val);

    // Now attempt to atomically change the value to the new one
    match ATOMIC.compare_exchange_weak(val, new, Ordering::Acquire, Ordering::Relaxed) {
        // Success
        (true, _) => break,

        // Failure, we get the current value back. Use that to try again.
        (false, old) => val = old,
    }
}
