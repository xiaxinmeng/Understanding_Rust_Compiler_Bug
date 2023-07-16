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
  --> src/tools/tidy/src/mir_opt_tests.rs:13:9
   |
12 |     walk_no_read(
   |     ------------ arguments to this function are incorrect
13 |         &path.join("mir-opt"),
   |         ^^^^^^^^^^^^^^^^^^^^^ expected `&[&Path]`, found `&PathBuf`
   |
   = note: expected reference `&[&Path]`
              found reference `&PathBuf`
  --> src/tools/tidy/src/walk.rs:67:15
   |
67 | pub(crate) fn walk_no_read(
   |               ^^^^^^^^^^^^
   |               ^^^^^^^^^^^^
68 |     paths: &[&Path],

For more information about this error, try `rustc --explain E0308`.
error: could not compile `tidy` due to previous error
warning: build failed, waiting for other jobs to finish...
