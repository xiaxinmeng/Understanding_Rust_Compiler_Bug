plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 6a1092056441652fe5fe5c5b422644951e6b99ce and d8f839c3807777957b1efa6372a32b8a2232716f
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
.....     (66/66)


/checkout/src/test/rustdoc-gui/search-result-display.goml search-result-display... FAILED
[ERROR] (line 12) Error: Evaluation failed: The following errors happened (for selector `.search-results div.desc`): [expected `570px` for key `width`, found `556.453px` (or `556px`)]: for command `assert-css: (".search-results div.desc", {"width": "570px"})`

/checkout/src/test/rustdoc-gui/toggle-docs-mobile.goml toggle-docs-mobile... FAILED
[ERROR] (line 7) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".top-doc", {"open": ""})`
[ERROR] (line 28) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".top-doc", {"open": ""})`
Build completed unsuccessfully in 0:00:16
