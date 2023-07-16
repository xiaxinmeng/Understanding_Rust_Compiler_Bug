plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling rustc-demangle v0.1.21
error[E0425]: cannot find value `other` in this scope
    --> library/alloc/src/rc.rs:1635:24
     |
1635 |         if self.ptr != other.ptr {

error[E0425]: cannot find value `other` in this scope
    --> library/alloc/src/rc.rs:2575:24
     |
     |
2575 |         if self.ptr != other.ptr {

error[E0425]: cannot find value `other` in this scope
    --> library/alloc/src/sync.rs:1533:24
     |
     |
1533 |         if self.ptr != other.ptr {

error[E0425]: cannot find value `other` in this scope
    --> library/alloc/src/sync.rs:2347:24
     |
     |
2347 |         if self.ptr != other.ptr {

For more information about this error, try `rustc --explain E0425`.
error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
