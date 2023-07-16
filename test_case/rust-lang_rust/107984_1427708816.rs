plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
error: expect test failed
error: test failed, to rerun pass `--lib`
   --> crates/proc-macro-srv/src/tests/mod.rs:51:9

You can update all `expect!` tests by running:

    env UPDATE_EXPECT=1 cargo test

To update a single test, place the cursor on `expect` token and use `run` feature of rust-analyzer.
Expect:
----
SUBTREE $$ 4294967295 4294967295
  IDENT   ident 4294967295
  IDENT   ident 4294967295
  PUNCH   , [alone] 4294967295
  SUBTREE [] 4294967295 4294967295

Actual:
----
SUBTREE $$ 4294967295 4294967295
SUBTREE $$ 4294967295 4294967295
  IDENT   ident 4294967295
  PUNCH   , [alone] 4294967295

Diff:
----
SUBTREE $$ 4294967295 4294967295
SUBTREE $$ 4294967295 4294967295
  IDENT   ident 4294967295
  PUNCH   , [alone] 4294967295
  SUBTREE [] 4294967295 4294967295



failures:
