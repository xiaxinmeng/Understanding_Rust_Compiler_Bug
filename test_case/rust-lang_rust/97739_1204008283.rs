
error: non-binding let on a synchronization lock
   --> src/main.rs:106:9
    |
106 |         let _ = lock.lock().unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[deny(let_underscore_lock)]` on by default
help: consider binding to an unused variable to avoid immediately dropping the value
    |
106 |         let _unused = lock.lock().unwrap();
    |             ~~~~~~~
help: consider immediately dropping the value
    |
106 |         drop(lock.lock().unwrap());
    |         ~~~~~                    +
