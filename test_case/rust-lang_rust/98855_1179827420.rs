plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c396bb3b8a16b1f2762b7c6078dc3e023f6a2493 and bfc6f26ecca8133bfd22e604c14e8d580bafb6b6
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
......     (66/66)


/checkout/src/test/rustdoc-gui/search-result-display.goml search-result-display... FAILED
[ERROR] (line 19) Error: Evaluation failed: The following errors happened (for selector `#crate-search`): [expected `222px` for key `width`, found `223.797px` (or `224px`)]: for command `assert-css: ("#crate-search", {"width": "222px"})`
[ERROR] (line 20) "#search-settings .search-results-title" not found: for command `compare-elements-position-near: (
    "#crate-search",
    "#search-settings .search-results-title",
    {"y": 5},
)`
[ERROR] (line 33) Error: Evaluation failed: The following errors happened (for selector `#crate-search`): [expected `640px` for key `width`, found `526.828px` (or `527px`)]: for command `assert-css: ("#crate-search", {"width": "640px"})`
[ERROR] (line 34) "#search-settings .search-results-title" not found: for command `compare-elements-position-near-false: (
    "#crate-search",
    "#search-settings .search-results-title",
    {"y": 5},

Build completed unsuccessfully in 0:00:22
