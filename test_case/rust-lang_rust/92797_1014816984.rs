plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a34c0797528172ede89480e3033f7a5e71ea4735 and 92d8dc6eb8c31db291769b35c06ca4c04336b55f
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
................... (50/57)
.......    (57/57)


/checkout/src/test/rustdoc-gui/sidebar.goml sidebar... FAILED
[ERROR] (line 27) Error: Evaluation failed: expected `389` for property `scrollTop` for selector `html`, found `371`: for command `assert-property: ("html", {"scrollTop": "389"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16
