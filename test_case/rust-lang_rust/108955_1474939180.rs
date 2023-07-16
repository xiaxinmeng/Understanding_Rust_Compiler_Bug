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
   Compiling miniz_oxide v0.5.3
error: the feature `staged_api` is internal to the compiler or standard library
  --> library/stdarch/crates/std_detect/src/lib.rs:16:12
   |
16 | #![feature(staged_api, stdsimd, doc_cfg, allow_internal_unstable)]
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default
error: the feature `allow_internal_unstable` is internal to the compiler or standard library
  --> library/stdarch/crates/std_detect/src/lib.rs:16:42
   |
   |
16 | #![feature(staged_api, stdsimd, doc_cfg, allow_internal_unstable)]
   |
   |
   = note: using it is strongly discouraged
error: could not compile `std_detect` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:39
cat: /tmp/toolstate/toolstates.json: No such file or directory
