plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ea5fa17998d7768664129294277986f7bad27923 and 4e45a0627793923b0e298e8ad67b70ba51c1eb9a
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/63)
..        (63/63)


/checkout/src/test/rustdoc-gui/search-result-display.goml search-result-display... FAILED
[ERROR] (line 5) TimeoutError: waiting for selector "#search-settings" failed: timeout 30000ms exceeded: for command `wait-for: "#search-settings"`
Build completed unsuccessfully in 0:00:41
