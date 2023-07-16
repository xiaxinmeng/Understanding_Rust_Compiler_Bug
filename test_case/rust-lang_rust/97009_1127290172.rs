plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: unknown lint: `unused_macro_rules`
  --> library/std/src/path/tests.rs:10:9
   |
10 | #[allow(unused_macro_rules)]
   |         ^^^^^^^^^^^^^^^^^^ help: did you mean: `unused_macros`
   |
   = note: `-D unknown-lints` implied by `-D warnings`
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:48
