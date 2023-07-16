plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

error: expect test failed
   --> compiler/rustc_lexer/src/tests.rs:237:9

You can update all `expect!` tests by running:

    env UPDATE_EXPECT=1 cargo test

To update a single test, place the cursor on `expect` token and use `run` feature of rust-analyzer.
Expect:
----
----
Token { kind: Lifetime { starts_with_number: false }, len: 4 }
----


error: test failed, to rerun pass `-p rustc_lexer --lib`
----
----
Token { kind: Lifetime { starts_with_number: false, contains_emoji: false }, len: 4 }
----

Diff:
----
----
Token { kind: Lifetime { starts_with_number: false, contains_emoji: false }, len: 4 }
----



