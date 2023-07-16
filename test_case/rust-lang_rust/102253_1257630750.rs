plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f3fafbb006ee98635874f73e480655912b465e65 and e37c7d07b13150d1bfea6608479708cff81017d5
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.................. (70/75)
.....      (75/75)


/checkout/src/test/rustdoc-gui/sidebar-mobile-scroll.goml sidebar-mobile-scroll... FAILED
[ERROR] (line 9) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `645`]: for command `assert-window-property: {"pageYOffset": "643"}`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `645`]: for command `assert-window-property: {"pageYOffset": "643"}`
[ERROR] (line 31) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `645`]: for command `assert-window-property: {"pageYOffset": "643"}`

/checkout/src/test/rustdoc-gui/sidebar-source-code-display.goml sidebar-source-code-display... FAILED
[ERROR] (line 230) Error: The following CSS properties still don't match: [width: (`0px` != `500px`)]: for command `wait-for-css: (".sidebar", {"width": "500px"})`
Build completed unsuccessfully in 0:02:20
