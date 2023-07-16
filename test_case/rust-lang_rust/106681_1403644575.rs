plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7873766cb4a6a0bb00c54496556b38db3fffa5d6)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling autocfg v1.1.0
   Compiling num-traits v0.2.12
   Compiling unstable-book-gen v0.1.0 (/checkout/src/tools/unstable-book-gen)
    Finished release [optimized] target(s) in 2.39s
thread 'main' panicked at 'failed to parse JSON docs: Error("missing field `variant_kind`", line: 1, column: 2997)', src/tools/tidy/src/unstable_book.rs:54:59
Build completed unsuccessfully in 0:21:52
