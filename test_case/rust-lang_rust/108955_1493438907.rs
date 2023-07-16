plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1081 tests
---
  |
2 | #![feature(intrinsics)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0093 (line 1948) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0093 (line 1948) stdout ----
error: the feature `intrinsics` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1949:12
  |
2 | #![feature(intrinsics)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0094 (line 1981) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0094 (line 1981) stdout ----
error: the feature `intrinsics` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1981:12
  |
2 | #![feature(intrinsics)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0211::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 3961) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0211::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 3961) stdout ----
error: the feature `intrinsics` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:3961:12
  |
2 | #![feature(intrinsics)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0264 (line 4895) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0264 (line 4895) stdout ----
error: the feature `lang_items` is internal to the compiler or standard library
  |
2 | #![feature(lang_items)]
  |            ^^^^^^^^^^
  |
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0539 (line 11367) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0539 (line 11367) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11367:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0542 (line 11443) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0542 (line 11443) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11443:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0544 (line 11521) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0544 (line 11521) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11521:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0543 (line 11486) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0543 (line 11486) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11486:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0546 (line 11591) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0546 (line 11591) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11591:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0547 (line 11627) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0547 (line 11627) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11627:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0549 (line 11668) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0549 (line 11668) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11668:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0545 (line 11555) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0545 (line 11555) stdout ----
error: the feature `staged_api` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11555:12
  |
2 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0622 (line 13475) stdout ----
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0622 (line 13475) stdout ----
error: the feature `intrinsics` is internal to the compiler or standard library
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13476:12
  |
2 | #![feature(intrinsics)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.

