plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 1409c015b44a4d4d38bef2250b2a37c17b8b7463 and 749efebcc84deac21dc8ffce5d78a550bc9ba9a1
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (50/56)
......     (56/56)


/checkout/src/test/rustdoc-gui/headings.goml headings... FAILED
[ERROR] (line 18) Error: Evaluation failed: expected `1px` for key `border-bottom-width` for selector `.main-heading`, found `0px`: for command `assert-css: (".main-heading", {"border-bottom-width": "1px"})`

/checkout/src/test/rustdoc-gui/toggle-docs-mobile.goml toggle-docs-mobile... FAILED
[ERROR] (line 5) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".top-doc", {"open": ""})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:14
