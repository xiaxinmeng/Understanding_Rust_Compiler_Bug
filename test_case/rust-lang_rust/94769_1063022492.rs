plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 10dccdc7fcbdc64ee9efe2c1ed975ab8c1d61287 and 28f0eef97942d0a708d641264e1b39e79177f2a2
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/mobile.goml mobile... FAILED
[ERROR] (line 27) Error: Evaluation failed: assert didn't fail: for command `compare-elements-position-near-false: ("#preferred-light-theme .setting-name", "#preferred-light-theme .choice", {"y": 16})`

/checkout/src/test/rustdoc-gui/search-tab-selection-if-current-is-empty.goml search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:46
