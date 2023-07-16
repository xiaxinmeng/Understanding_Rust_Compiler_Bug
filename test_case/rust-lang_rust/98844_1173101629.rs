plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f99f9e48ed77a99747c6d07b42fdfe500f1a7de0 and f1fd222d9b401b5c18ff94d255916194cafaebf1
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
.....     (66/66)


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 6) Error: The following CSS selector "#titles" was not found: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:46
