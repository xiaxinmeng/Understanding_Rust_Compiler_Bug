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
test result: ok. 236 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out; finished in 14.64s

 finished in 14.714 seconds
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
thread 'main' panicked at 'invalid line in /checkout/tests/codegen/issues/issue-37945.rs: ignore-32-bit LLVM has a bug with them', src/tools/compiletest/src/header.rs:944:42
Build completed unsuccessfully in 0:11:48
