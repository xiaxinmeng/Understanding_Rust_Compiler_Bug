plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

   Doc-tests rustc_mir_build

running 27 tests
iiiiiiiiiiiii.F............

---- src/thir/pattern/usefulness.rs - thir::pattern::usefulness::Witness (line 685) stdout ----
error: expected identifier, found `:`
 --> src/thir/pattern/usefulness.rs:689:9
 --> src/thir/pattern/usefulness.rs:689:9
  |
6 | match (p: Pair) {

error: aborting due to previous error


Some expected error codes were not found: ["E0004"]
failures:
    src/thir/pattern/usefulness.rs - thir::pattern::usefulness::Witness (line 685)

test result: FAILED. 13 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out; finished in 0.21s
test result: FAILED. 13 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out; finished in 0.21s

error: doctest failed, to rerun pass `-p rustc_mir_build --doc`
