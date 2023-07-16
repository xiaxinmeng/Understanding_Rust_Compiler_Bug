
error: non-binding let on a synchronization lock
   --> src/main.rs:106:9
    |
106 |         let _ = lock.lock().unwrap();
    |             ^   ^^^^^^^^^^^^^^^^^^^^ this lock is not assigned to a binding and is immediately dropped
    |             |
    |             this binding will immediately drop the value assigned to it
    |
    = note: `#[deny(let_underscore_lock)]` on by default
help: consider binding to an unused variable to dropp the value at the end of its scope
    |
106 |         let _lock_guard = lock.lock().unwrap();
    |             ~~~~~~~~~~~
help: consider explicitly immediately dropping the value
    |
106 |         drop(lock.lock().unwrap());
    |         ~~~~~                    +
