plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: argument never used
   --> library/std/src/fs/tests.rs:475:53
    |
475 |         let msg_str = format!("{prefix}{}", prefix, n.to_string());
    |                               ------------          ^^^^^^^^^^^^^ argument never used
    |                               formatting specifier missing

error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
