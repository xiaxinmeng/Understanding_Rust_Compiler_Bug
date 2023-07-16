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

failures:

---- builder::tests::dist::build_with_empty_host stdout ----
`x.py dist` run with empty `host` parameter. Either set it or leave it out for default value.
thread 'builder::tests::dist::build_with_empty_host' panicked at 'status code: 1', lib.rs:1583:9

error: test failed, to rerun pass `--lib`
---- builder::tests::dist::dist_with_empty_host stdout ----
---- builder::tests::dist::dist_with_empty_host stdout ----
`x.py dist` run with empty `host` parameter. Either set it or leave it out for default value.
thread 'builder::tests::dist::dist_with_empty_host' panicked at 'status code: 1', lib.rs:1583:9

failures:
    builder::tests::dist::build_with_empty_host
    builder::tests::dist::dist_with_empty_host
