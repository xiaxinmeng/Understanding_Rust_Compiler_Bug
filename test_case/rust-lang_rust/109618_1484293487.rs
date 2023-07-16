plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:13c1b4e09b845ddb9664cee13d03879444a1054d)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
.....F...............................

failures:

---- bit_set::tests::chunked_bitset_into_bitset_operations stdout ----
thread 'bit_set::tests::chunked_bitset_into_bitset_operations' panicked at 'not implemented: Stop being lazy', compiler/rustc_index/src/bit_set/fixed.rs:492:21


failures:
failures:
error: test failed, to rerun pass `-p rustc_index --lib`

test result: FAILED. 36 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 233.60ms

Build completed unsuccessfully in 0:18:59
