plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e012a191d768adeda1ee36a99ef8b92d51920154 and c4ab1e416c3efd0922a3e0f71b2518a5a6d84788
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/56)
......     (56/56)


/checkout/src/test/rustdoc-gui/hash-item-expansion.goml hash-item-expansion... FAILED
[ERROR] (line 10) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: ("#implementations + details", {"open": ""})`

/checkout/src/test/rustdoc-gui/item-info-width.goml item-info-width... FAILED
[ERROR] (line 6) Error: Evaluation failed: expected `807px` for key `width` for selector `.item-info`, found `816px`: for command `assert-css: (".item-info", {"width": "807px"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:18
