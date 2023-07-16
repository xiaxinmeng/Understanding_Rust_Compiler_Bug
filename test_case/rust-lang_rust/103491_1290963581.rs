plain
Successfully built 8f8049b7e977
Successfully tagged rust-ci:latest
Built container sha256:8f8049b7e977661d823421d6799e5b3264a45bb847841e49de96f231c6c9ae59
Uploading finished image to https://ci-caches.rust-lang.org/docker/5113f42bfd9b1c95a1d3883587ba8dad4c66787bb247c62a9135b39daac8eb518d79354d0685bc057d2ca9275608acb8acc0d283225531823eccf30a4e542d9b
upload failed: - to s3://rust-lang-ci-sccache2/docker/5113f42bfd9b1c95a1d3883587ba8dad4c66787bb247c62a9135b39daac8eb518d79354d0685bc057d2ca9275608acb8acc0d283225531823eccf30a4e542d9b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1044 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0790 (line 16924) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0760 (line 16100) stdout ----
error[E0658]: `async fn` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
  |
  |
6 |     async fn new(i: &'a i32) -> Self {
  |                                 ^^^^ help: consider spelling out the type instead: `S<'a>`
  = note: see issue #103532 <https://github.com/rust-lang/rust/issues/103532> for more information
  = help: add `#![feature(impl_trait_projections)]` to the crate attributes to enable

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Some expected error codes were not found: ["E0760"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0760 (line 16100)

test result: FAILED. 1009 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out; finished in 9.22s
