plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 38b72154ded23847cd08a796d0c6708b5efac265 and 95bf875011c9b6e997d35dc49205359036592f13
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
.....     (66/66)


/checkout/src/test/rustdoc-gui/search-tab-change-title-fn-sig.goml search-tab-change-title-fn-sig... FAILED
[ERROR] (line 6) Error: The following CSS selector "#titles" was not found: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:46
