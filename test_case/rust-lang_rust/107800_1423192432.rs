plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:cb2b9920774a63bd54b3676f2b669ea1e777a91e)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
test setup::tests::check_matching_settings_hash ... FAILED

failures:

---- setup::tests::check_matching_settings_hash stdout ----
thread 'setup::tests::check_matching_settings_hash' panicked at 'assertion failed: `(left == right)`
  left: `"1993683aad3ecd43bea6c65e545a15035242a4ea6d207c2b607777ec05a1af4c"`,
 right: `"ea67e259dedf60d4429b6c349a564ffcd1563cf41c920a856d1f5b16b4701ac8"`: Update `SETTINGS_HASHES` with the new hash of `src/etc/vscode_settings.json`', setup/tests.rs:9:5

failures:
    setup::tests::check_matching_settings_hash

