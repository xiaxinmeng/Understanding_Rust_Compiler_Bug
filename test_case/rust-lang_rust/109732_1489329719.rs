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
   Compiling rand v0.8.5
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
error: calls to `std::mem::drop` with a reference instead of an owned value
    |
    |
378 |             s.spawn(|| drop(x));
    |
note: argument has type `&u8`
   --> library/std/src/thread/tests.rs:378:29
    |
    |
378 |             s.spawn(|| drop(x));
    |                             ^
    = note: `-D drop-ref` implied by `-D warnings`
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:15:29
