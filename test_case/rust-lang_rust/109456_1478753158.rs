plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
doc tests for: /checkout/src/doc/rustdoc/src/write-documentation/linking-to-items-by-name.md
doc tests for: /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/write-documentation/re-exports.md" "--test-args" ""

stdout ----

running 7 tests
running 7 tests
test /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports (line 17) ... ignored
test /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports (line 41) ... ignored
test /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_rules (line 55) ... FAILED
test /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_with_ (line 91) ... FAILED
test /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports (line 25) ... FAILED
test /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_rules (line 72) ... FAILED
test /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports (line 6) ... ok
failures:


---- /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_rules (line 55) stdout ----
error[E0432]: unresolved import `super::private_module`
 --> /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md:62:20
9 |     pub use super::private_module::Public;
9 |     pub use super::private_module::Public;
  |                    ^^^^^^^^^^^^^^ maybe a missing crate `private_module`?
  |
  = help: consider adding `extern crate private_module` to use the `private_module` crate
error[E0432]: unresolved import `self::public_mod`
  --> /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md:66:15
   |
13 | pub use self::public_mod::Public;
13 | pub use self::public_mod::Public;
   |               ^^^^^^^^^^ maybe a missing crate `public_mod`?
   |
   = help: consider adding `extern crate public_mod` to use the `public_mod` crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_with_ (line 91) stdout ----
error[E0432]: unresolved import `self::public_mod`
 --> /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md:97:15
8 | pub use self::public_mod::Public;
8 | pub use self::public_mod::Public;
  |               ^^^^^^^^^^ maybe a missing crate `public_mod`?
  |
  = help: consider adding `extern crate public_mod` to use the `public_mod` crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports (line 25) stdout ----
error[E0432]: unresolved import `crate::sub_module1`
  --> /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md:35:16
12 | pub use crate::sub_module1::Foo;
12 | pub use crate::sub_module1::Foo;
   |                ^^^^^^^^^^^ maybe a missing crate `sub_module1`?
   |
   = help: consider adding `extern crate sub_module1` to use the `sub_module1` crate
error[E0432]: unresolved import `crate::sub_module2`
  --> /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md:36:16
   |
   |
13 | pub use crate::sub_module2::AnotherFoo;
   |                ^^^^^^^^^^^ maybe a missing crate `sub_module2`?
   |
   = help: consider adding `extern crate sub_module2` to use the `sub_module2` crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_rules (line 72) stdout ----
error[E0432]: unresolved import `self::public_mod`
  --> /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md:82:15
12 | pub use self::public_mod::Public;
12 | pub use self::public_mod::Public;
   |               ^^^^^^^^^^ maybe a missing crate `public_mod`?
   |
   = help: consider adding `extern crate public_mod` to use the `public_mod` crate
error[E0432]: unresolved import `self::Hidden`
  --> /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md:84:9
   |
14 | pub use self::Hidden;
14 | pub use self::Hidden;
   |         ^^^^^^^^^^^^ no `Hidden` in the root
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports (line 25)
    /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_rules (line 55)
    /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_rules (line 72)
    /checkout/src/doc/rustdoc/src/write-documentation/re-exports.md - Re_exports::Inlining_with_ (line 91)
test result: FAILED. 1 passed; 4 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.10s


stderr ----
