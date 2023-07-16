plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
.......... (90/98)
.......   (98/98)


/checkout/tests/rustdoc-gui/search-tab-change-title-fn-sig.goml search-tab-change-title-fn-sig... FAILED
[ERROR] (line 36) "#results > .search-results.active > a:nth-of-type(1):focus" not found: for command `assert: "#results > .search-results.active > a:nth-of-type(1):focus"`
Build completed unsuccessfully in 0:01:57
