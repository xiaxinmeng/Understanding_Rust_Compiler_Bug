
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling term v0.0.0 (file:///home/nmatsakis/versioned/rust-4/src/libterm)
   Compiling getopts v0.2.15
   Compiling test v0.0.0 (file:///home/nmatsakis/versioned/rust-4/src/libtest)
error: use of unstable library feature 'termination_trait' (see issue #43301)
  --> libtest/lib.rs:73:5
   |
73 | use std::Termination;
   |     ^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(termination_trait)] to the crate attributes to enable

error: use of unstable library feature 'termination_trait' (see issue #43301)
   --> libtest/lib.rs:327:30
    |
327 | pub fn assert_test_result<T: Termination>(result: T) {
    |                              ^^^^^^^^^^^
    |
    = help: add #![feature(termination_trait)] to the crate attributes to enable

error: use of unstable library feature 'termination_trait' (see issue #43301)
   --> libtest/lib.rs:328:5
    |
328 |     Termination::assert_unit_test_successful(result)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add #![feature(termination_trait)] to the crate attributes to enable
