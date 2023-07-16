plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b29f1f8d3b3f419522b0eb149b664e5325f7b306)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
test result: ok. 40 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.32s

 finished in 0.428 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
thread 'main' panicked at 'missing LLVM component: loongarch', src/tools/compiletest/src/header.rs:1128:17
Build completed successfully in 0:11:52
