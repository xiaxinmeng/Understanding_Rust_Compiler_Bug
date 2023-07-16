plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 28d0e75269ad092662fef27f44c6aa029c376d49 and 69bd5e82fb4cafde45135bf90a4e0b8b8f082542
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.........  (49/49)


header-size... FAILED
[ERROR] (line 27) Error: Evaluation failed: expected `16px` for key `font-size` for selector `#impl > h3.code-header`, found `17.6px`: for command `assert-css: ("#impl > h3.code-header", {"font-size": "16px"}) // 1em`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:18
