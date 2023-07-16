plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 57ee5cf5a93923dae9c98bffb11545fc3a31368d and 6ee7595c8f518f263d28bfe83e79822be5e0243b
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/75)
.....      (75/75)


/checkout/src/test/rustdoc-gui/sidebar-mobile-scroll.goml sidebar-mobile-scroll... FAILED
[ERROR] (line 9) Error: Evaluation failed: The following errors happened: [Expected `645` for property `pageYOffset`, found `639`]: for command `assert-window-property: {"pageYOffset": "645"}`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened: [Expected `645` for property `pageYOffset`, found `639`]: for command `assert-window-property: {"pageYOffset": "645"}`
[ERROR] (line 31) Error: Evaluation failed: The following errors happened: [Expected `645` for property `pageYOffset`, found `639`]: for command `assert-window-property: {"pageYOffset": "645"}`
Build completed unsuccessfully in 0:02:04
