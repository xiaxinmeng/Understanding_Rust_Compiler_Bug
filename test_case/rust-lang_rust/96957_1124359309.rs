plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1039 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0785 (line 16746) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0726::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 14980) stdout ----
error[E0670]: `async fn` is not permitted in Rust 2015
  |
  |
8 | async fn create(content: Content) { // error: implicit elided
  | ^^^^^ to use `async fn`, switch to Rust 2018 or later
  |
  = help: pass `--edition 2021` to `rustc`
  = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0433]: failed to resolve: maybe a missing crate `futures`?
  |
3 | use futures::executor::block_on;
3 | use futures::executor::block_on;
  |     ^^^^^^^ maybe a missing crate `futures`?
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0670.
For more information about an error, try `rustc --explain E0433`.
