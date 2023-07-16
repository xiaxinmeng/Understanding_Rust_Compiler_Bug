plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d72b7d2d2a64f5f77b919a1428873b4d4149f60d and 53456b0b718f8893c9d9105556c4218d899d7156
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/98)
........   (98/98)


/checkout/src/test/rustdoc-gui/sidebar-mobile-scroll.goml sidebar-mobile-scroll... FAILED
[ERROR] (line 9) Error: Evaluation failed: The following errors happened: [Expected `627` for property `pageYOffset`, found `907`]: for command `assert-window-property: {"pageYOffset": "627"}`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened: [Expected `627` for property `pageYOffset`, found `907`]: for command `assert-window-property: {"pageYOffset": "627"}`
[ERROR] (line 31) Error: Evaluation failed: The following errors happened: [Expected `627` for property `pageYOffset`, found `907`]: for command `assert-window-property: {"pageYOffset": "627"}`
Build completed unsuccessfully in 0:02:32
