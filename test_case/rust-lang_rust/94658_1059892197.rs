plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0cbef1c6a7f4ee33cd41d91778544c5399f10f39 and 0ae39dbbddb0be3f61675c37abbf828ee81e7a99
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/search-tab-selection-if-current-is-empty.goml search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:44
