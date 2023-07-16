plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
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

running 1078 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0698 (line 14919) stdout ----
error[E0282]: type annotations needed
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:14923:5
  |
6 |     bar().await; // error: cannot infer type for `T`
  |     ^^^ cannot infer type of the type parameter `T` declared on the function `bar`
help: consider specifying the generic argument
  |
  |
6 |     bar::<T>().await; // error: cannot infer type for `T`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Some expected error codes were not found: ["E0698"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0698 (line 14919)

test result: FAILED. 1028 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out; finished in 9.76s
