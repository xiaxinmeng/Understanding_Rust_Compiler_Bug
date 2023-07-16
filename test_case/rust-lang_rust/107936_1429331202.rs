plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `TypeVisitable`
 --> compiler/rustc_middle/src/ty/instance.rs:3:84
  |
3 | use crate::ty::{self, Binder, Ty, TyCtxt, TyKind, TypeFoldable, TypeSuperFoldable, TypeVisitable};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:44
cat: /tmp/toolstate/toolstates.json: No such file or directory
