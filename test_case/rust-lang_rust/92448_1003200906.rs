plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b60e32c82864d841e87359333af1e6d1f9cff9ee and 6eb702c6718e417ef236e798607d8ed18c8183c9
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/54)
....       (54/54)


/checkout/src/test/rustdoc-gui/item-info-width.goml item-info-width... FAILED
[ERROR] (line 7) Error: Evaluation failed: expected `341px` for key `width` for selector `.item-info .stab`, found `349px`: for command `assert-css: (".item-info .stab", {"width": "341px"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:14
