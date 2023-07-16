plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 3d237ab52920924694f6fc3e47ee8588514bfa70 and 2127c866dba4e4bd0e6f4bb69d100bf8ca3c0e35
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/62)
.         (62/62)


/checkout/src/test/rustdoc-gui/search-tab-selection-if-current-is-empty.goml search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:42
