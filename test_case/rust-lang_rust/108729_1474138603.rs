plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  |
3 | use rustc_middle::mir::coverage::*;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error: could not compile `rustc_mir_transform` due to previous error
