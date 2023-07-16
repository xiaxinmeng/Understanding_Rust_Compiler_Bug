plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: use of function `env::set_var` that will be deprecated in future version 1.58.0: unclear soundness on POSIX, use `libc::setenv` or `std::process::Command::env` instead
   --> library/std/src/process/tests.rs:271:10
    |
271 |     env::set_var("RUN_TEST_NEW_ENV2", "456");
    |
    |
    = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of function `env::remove_var` that will be deprecated in future version 1.58.0: unclear soundness on POSIX, use `libc::unsetenv` or `std::process::Command::env_remove` instead
   --> library/std/src/process/tests.rs:273:10
    |
273 |     env::remove_var("RUN_TEST_NEW_ENV2");

error: could not compile `std` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
