plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5b9775fe17893cba641a071de7e0a7c8f478c41b and afc8e01cbeb5278f6d41fc4314d0709d318b7735
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
.....     (66/66)


/checkout/src/test/rustdoc-gui/sidebar-source-code-display.goml sidebar-source-code-display... FAILED
[ERROR] (line 39) Error: Evaluation failed: The following errors happened (for selector `#source-sidebar details[open] > .files a.selected`): [expected `rgb(255, 255, 255)` for key `background-color`, found `rgb(224, 224, 224)`]: for command `assert-css: (
    "#source-sidebar details[open] > .files a.selected",
    {"color": "rgb(0, 0, 0)", "background-color": "rgb(255, 255, 255)"},
)`
[ERROR] (line 71) Error: Evaluation failed: The following errors happened (for selector `#source-sidebar details[open] > .files > a.selected`): [expected `rgb(51, 51, 51)` for key `background-color`, found `rgb(68, 68, 68)`]: for command `assert-css: (
    "#source-sidebar details[open] > .files > a.selected",
    {"color": "rgb(221, 221, 221)", "background-color": "rgb(51, 51, 51)"},

Build completed unsuccessfully in 0:00:20
