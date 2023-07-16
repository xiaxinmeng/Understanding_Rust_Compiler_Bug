plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:917222de331afc95ef8d3a6300048017039b2b08)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

     Running unittests src/lib.rs (obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_index-d1bb8f5ca6048919)

running 37 tests
error: test failed, to rerun pass `-p rustc_index --lib`
.......F.............................
failures:

---- bit_set::tests::chunked_bitset_into_bitset_operations stdout ----
---- bit_set::tests::chunked_bitset_into_bitset_operations stdout ----
thread 'bit_set::tests::chunked_bitset_into_bitset_operations' panicked at 'not implemented: Stop being lazy', compiler/rustc_index/src/bit_set/fixed.rs:446:21


failures:
    bit_set::tests::chunked_bitset_into_bitset_operations
