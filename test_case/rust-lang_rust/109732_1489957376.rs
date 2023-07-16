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
............ii.................F......................................

failures:

---- src/drop_forget_useless.rs - drop_forget_useless::DROP_REF (line 14) stdout ----
error[E0425]: cannot find value `mutex` in this scope
 --> src/drop_forget_useless.rs:16:22
4 | let mut lock_guard = mutex.lock();
  |                      ^^^^^ not found in this scope

error: aborting due to previous error
---
    src/drop_forget_useless.rs - drop_forget_useless::DROP_REF (line 14)

test result: FAILED. 67 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 2.01s

error: doctest failed, to rerun pass `-p rustc_lint --doc`
