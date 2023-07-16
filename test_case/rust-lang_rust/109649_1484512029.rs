plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:13c1b4e09b845ddb9664cee13d03879444a1054d)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0433]: failed to resolve: could not find `native` in the crate root
  --> config/tests.rs:14:15
   |
14 |     if crate::native::is_ci_llvm_modified(&parse("")) {
   |               ^^^^^^ could not find `native` in the crate root
For more information about this error, try `rustc --explain E0433`.
error: could not compile `bootstrap` due to previous error
Build completed unsuccessfully in 0:28:23
