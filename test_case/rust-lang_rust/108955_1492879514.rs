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
  |
2 | #![feature(rustc_attrs)]
  |            ^^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.


failures:
    src/assert_module_sources.rs - assert_module_sources (line 7)

test result: FAILED. 0 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 27.94ms

error: doctest failed, to rerun pass `-p rustc_incremental --doc`
