plain
Set({test::library/test}) not skipped for "bootstrap::test::Crate" -- not in [src/tools/tidy]
 finished in 180.171 seconds
Testing test stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling test v0.0.0 (/checkout/library/test)
error: call to unsafe function that has been deprecated as being safe and now requires an unsafe block (error E0FIXME)
  --> library/test/src/term/terminfo/searcher/tests.rs:16:5
   |
16 |     env::set_var("TERMINFO_DIRS", ":");
   |
   |
   = note: `-D deprecated-safe` implied by `-D warnings`


error: call to unsafe function that has been deprecated as being safe and now requires an unsafe block (error E0FIXME)
  --> library/test/src/term/terminfo/searcher/tests.rs:18:5
   |
18 |     env::remove_var("TERMINFO_DIRS");
   |
   = note: reason

error: could not compile `test` due to 2 previous errors
