rust
let guard = thread::guarded(move || {
    // ...
});

// Join the thread.
// Panics if the thread ended with a panic.
drop(guard);
