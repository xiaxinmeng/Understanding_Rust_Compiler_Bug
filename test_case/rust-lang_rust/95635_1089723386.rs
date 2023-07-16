plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between bbe9d27b8ff36da56638aa43d6d0cdfdf89a4e57 and 7226ce367ac886172f13245bec9b784fab46b965
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:42
