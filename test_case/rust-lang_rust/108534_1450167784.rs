plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:e58f560dd5e72b61cf9aeebf432d862b23ac76a8)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s



failed to execute command: CARGO="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" "src/tools/rust-installer/test.sh"
error: No such file or directory (os error 2)

Build completed unsuccessfully in 0:37:03
