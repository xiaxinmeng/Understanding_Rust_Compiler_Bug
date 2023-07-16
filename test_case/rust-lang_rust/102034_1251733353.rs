plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 2019147c5642c08cdb9ad4cacd97dd1fa4ffa701 and 2bfb388a3bd57c435a44b9310566d068bd67ade2
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/73)
...        (73/73)


/checkout/src/test/rustdoc-gui/sidebar-mobile-scroll.goml sidebar-mobile-scroll... FAILED
[ERROR] (line 9) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `644`]: for command `assert-window-property: {"pageYOffset": "643"}`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `644`]: for command `assert-window-property: {"pageYOffset": "643"}`
[ERROR] (line 31) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `644`]: for command `assert-window-property: {"pageYOffset": "643"}`
Build completed unsuccessfully in 0:01:51
