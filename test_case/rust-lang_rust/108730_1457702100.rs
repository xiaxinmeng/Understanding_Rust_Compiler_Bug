plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
doc tests for: /checkout/src/doc/rustc/src/lints/index.md
doc tests for: /checkout/src/doc/rustc/src/lints/levels.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/lints/levels.md" "--test-args" ""

stdout ----

running 6 tests
---
test /checkout/src/doc/rustc/src/lints/levels.md - Lint_Levels::Configuring_warning_levels::Via_an_attribute (line 228) ... ok

failures:

---- /checkout/src/doc/rustc/src/lints/levels.md - Lint_Levels::Configuring_warning_levels::Capping_lints (line 249) stdout ----
error: this arithmetic operation will overflow
  |
3 |     100u8 << 10;
3 |     100u8 << 10;
  |     ^^^^^^^^^^^ attempt to shift left by `10_i32`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/src/doc/rustc/src/lints/levels.md - Lint_Levels::deny (line 74) stdout ----
---- /checkout/src/doc/rustc/src/lints/levels.md - Lint_Levels::deny (line 74) stdout ----
error: this arithmetic operation will overflow
  |
3 |     100u8 << 10;
3 |     100u8 << 10;
  |     ^^^^^^^^^^^ attempt to shift left by `10_i32`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
error: aborting due to previous error

Couldn't compile the test.

