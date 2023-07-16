plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5b8cf49c51833ee5d27ae2e8e179337dbb9f14d7 and ca3c07560e6da1adf890ddd838208bad545d6fe2
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
.....     (66/66)


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 6) Error: The following CSS selector "#titles" was not found: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:45
