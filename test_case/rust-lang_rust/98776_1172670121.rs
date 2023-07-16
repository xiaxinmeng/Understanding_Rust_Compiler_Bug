plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5b9775fe17893cba641a071de7e0a7c8f478c41b and 6fb496187ec3d06ef19f2d19c7ded2baf1a47f07
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/65)
....      (65/65)


/checkout/src/test/rustdoc-gui/sidebar-source-code-display.goml sidebar-source-code-display... FAILED
[ERROR] (line 185) Error: Evaluation failed: The following errors happened: [Expected `2519` for property `pageYOffset`, found `0`]: for command `assert-window-property: {"pageYOffset": "2519"}`
Build completed unsuccessfully in 0:00:20
