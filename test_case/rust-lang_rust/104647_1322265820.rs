plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 736c675d2ab65bcde6554e1b73340c2dbc27c85a and 999d40d8101aaba38fb0a99d64656bd6471b8e5a
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (80/88)
........   (88/88)


/checkout/src/test/rustdoc-gui/code-sidebar-toggle.goml code-sidebar-toggle... FAILED
[ERROR] (line 4) Error: Execution context was destroyed, most likely because of a navigation.: for command `wait-for: "#sidebar-toggle"`
Build completed unsuccessfully in 0:02:17
