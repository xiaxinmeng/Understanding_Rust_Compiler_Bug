
error: unsupported operation: unable to overwrite parts of a pointer in memory at alloc1390
   --> /home/r/src/rust/rustc/library/std/src/panicking.rs:401:13
    |
401 |             data.r = ManuallyDrop::new(f());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to overwrite parts of a pointer in memory at alloc1390
    |
    = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support

    = note: inside `std::panicking::try::do_call::<&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe, i32>` at /home/r/src/rust/rustc/library/std/src/panicking.rs:401:13
    = note: inside `std::panicking::try::<i32, &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>` at /home/r/src/rust/rustc/library/std/src/panicking.rs:365:19
