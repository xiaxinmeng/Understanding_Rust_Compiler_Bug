plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
error: unused variable: `bad`
  --> src/tools/tidy/src/extdeps.rs:11:27
   |
11 | pub fn check(root: &Path, bad: &mut bool) {
   |                           ^^^ help: if this is intentional, prefix it with an underscore: `_bad`
   |
   = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `tidy` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:17
