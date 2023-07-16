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

---- builder::tests::defaults::build_cross_compile stdout ----
thread 'builder::tests::defaults::build_cross_compile' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `1`: called assert_single_path on multiple paths', compile.rs:1011:21

---- builder::tests::defaults::build_default stdout ----
thread 'builder::tests::defaults::build_default' panicked at 'assertion failed: `(left == right)`
  left: `2`,
  left: `2`,
error: test failed, to rerun pass `--lib`
 right: `1`: called assert_single_path on multiple paths', compile.rs:1011:21
---- builder::tests::defaults::build_stage_0 stdout ----
---- builder::tests::defaults::build_stage_0 stdout ----
thread 'builder::tests::defaults::build_stage_0' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `1`: called assert_single_path on multiple paths', compile.rs:1011:21

failures:
    builder::tests::defaults::build_cross_compile
    builder::tests::defaults::build_default
