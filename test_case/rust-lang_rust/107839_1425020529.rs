plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ca7cab66e2f838703fe12775fbabb05754421ad8)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   |                ^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `init_env_logger`
   |
  ::: /checkout/compiler/rustc_log/src/lib.rs:56:1
   |
56 | pub fn init_env_logger(env: &str) -> Result<(), Error> {
   | ------------------------------------------------------ similarly named function `init_env_logger` defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/lib.rs - (line 15)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

error: doctest failed, to rerun pass `-p rustc_log --doc`
