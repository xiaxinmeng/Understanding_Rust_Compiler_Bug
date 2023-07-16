plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f1ce0e6a00593493a12e0e3662119786c761f375 and 5d7aa1612f9c88ec9b4a8cad00841a1ce0601984
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/54)
....       (54/54)


/checkout/src/test/rustdoc-gui/anchors.goml anchors... FAILED
[ERROR] (line 13) Error: Evaluation failed: expected `rgb(0, 0, 0)` for key `color` for selector `#toggle-all-docs`, found `rgb(56, 115, 173)`: for command `assert-css: ("#toggle-all-docs", {"color": "rgb(0, 0, 0)"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16
