plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b6097f2e1b2ca62e188ba53cf43bd66b06b36915 and c1651b53db2d2a3762b773ba06af0675d6d0e1c4
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (80/86)
......     (86/86)


/checkout/src/test/rustdoc-gui/code-sidebar-toggle.goml code-sidebar-toggle... FAILED
[ERROR] (line 4) Error: Execution context was destroyed, most likely because of a navigation.: for command `wait-for: "#sidebar-toggle"`
Build completed unsuccessfully in 0:02:14
