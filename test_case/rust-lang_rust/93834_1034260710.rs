plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e7aca895980f25f6d2d3c48e10fd04656764d1e4 and eee0044206efe22c38871f9df70358dc368104a0
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/59)
.........  (59/59)


/checkout/src/test/rustdoc-gui/item-info-width.goml item-info-width... FAILED
[ERROR] (line 7) Error: Evaluation failed: expected `340px` for key `width` for selector `.item-info .stab`, found `339.562px` (or `339px`): for command `assert-css: (".item-info .stab", {"width": "340px"})`
Build completed unsuccessfully in 0:00:18
