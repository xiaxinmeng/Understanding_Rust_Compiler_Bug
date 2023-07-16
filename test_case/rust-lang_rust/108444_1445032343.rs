plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
error: this must repeat at least once
  --> src/tools/error_index_generator/main.rs:25:18
   |
25 |                 $((stringify!($undocumented), None),)+

error[E0432]: unresolved import `crate::error_codes::error_codes`
 --> src/tools/error_index_generator/main.rs:6:5
  |
