plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 24a0eecf035235fa58c0b10bad1d3665947842e4 and f6513d225b478229c1a76b26662b1935bafbed2a
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:42
