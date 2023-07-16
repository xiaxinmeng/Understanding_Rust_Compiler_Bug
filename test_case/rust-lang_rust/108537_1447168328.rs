plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3aac11d64ed0b00fb4926ce853d58cf88de1213a)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
.......... (90/97)
......    (97/97)


/checkout/tests/rustdoc-gui/search-tab-change-title-fn-sig.goml search-tab-change-title-fn-sig... FAILED
[ERROR] (line 64) Error: Evaluation failed: The following errors happened: [`In Names (0)` doesn't start with `In Function Parameters` (for STARTS_WITH check)]: for command `assert-text: ("#search-tabs > button:nth-of-type(1)", "In Function Parameters", STARTS_WITH)`
Build completed unsuccessfully in 0:02:38
