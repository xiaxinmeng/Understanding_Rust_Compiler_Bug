plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
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
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0794::Additional_information (line 17752) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0577 (line 12289) stdout ----
error[E0433]: failed to resolve: `Sea` is a struct, not a module
  |
  |
4 | pub (in crate::Sea) struct Shark; // error!
  |                ^^^ `Sea` is a struct, not a module
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
For more information about this error, try `rustc --explain E0433`.
Some expected error codes were not found: ["E0577"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0577 (line 12289)

test result: FAILED. 1031 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out; finished in 9.90s
