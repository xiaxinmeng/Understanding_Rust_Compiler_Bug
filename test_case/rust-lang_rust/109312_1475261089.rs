plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Testing rustdoc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling dissimilar v1.0.4
   Compiling expect-test v1.4.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0560]: struct `rustc_resolve::rustdoc::DocFragment` has no field named `parent_module`
  --> src/librustdoc/clean/types/tests.rs:13:9
13 |         parent_module: None,
   |         ^^^^^^^^^^^^^ `rustc_resolve::rustdoc::DocFragment` does not have this field
   |
   |
   = note: available fields are: `span`, `item_id`, `doc`, `kind`, `indent`
For more information about this error, try `rustc --explain E0560`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:24:55
