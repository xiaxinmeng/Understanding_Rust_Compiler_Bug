plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d13e8dd41d44a73664943169d5b7fe39b22c449f and 3d3422f4ca8cd16b188e990c4560cc2958deb082
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (50/58)
........   (58/58)


/checkout/src/test/rustdoc-gui/sidebar.goml sidebar... FAILED
[ERROR] (line 12) Error: Evaluation failed: expected `rgb(56, 115, 173)` for key `color` for selector `#all-types`, found `rgb(53, 109, 164)`: for command `assert-css: ("#all-types", {"color": "rgb(56, 115, 173)"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:17
