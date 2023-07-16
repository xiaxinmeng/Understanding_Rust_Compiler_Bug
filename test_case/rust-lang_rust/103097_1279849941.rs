plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b8c35ca26b191bb9a9ac669a4b3f4d3d52d97fb1 and 160d5ec81af8bc6c9eb7f9ed296c145a53f5b3ee
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/80)
.......... (80/80)


/checkout/src/test/rustdoc-gui/help-page.goml help-page... FAILED
[ERROR] (line 7) "#help" not found: for command `assert-css: ("#help", {"display": "block"})`
Build completed unsuccessfully in 0:01:52
