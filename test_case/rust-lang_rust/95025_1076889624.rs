plain
 finished in 2.735 seconds
Set({test::library/std}) not skipped for "bootstrap::test::Crate" -- not in [src/tools/tidy]
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/checkout/library/std)
error: call to unsafe function that has been deprecated as being safe and now requires an unsafe block (error E0FIXME)
   --> library/std/src/process/tests.rs:281:5
    |
281 |     env::set_var("RUN_TEST_NEW_ENV2", "456");
    |
    |
    = note: `-D deprecated-safe` implied by `-D warnings`


error: call to unsafe function that has been deprecated as being safe and now requires an unsafe block (error E0FIXME)
   --> library/std/src/process/tests.rs:283:5
    |
283 |     env::remove_var("RUN_TEST_NEW_ENV2");
    |
    = note: reason

error: could not compile `std` due to 2 previous errors
