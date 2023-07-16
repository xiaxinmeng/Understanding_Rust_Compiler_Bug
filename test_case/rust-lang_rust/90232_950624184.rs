plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 28d0e75269ad092662fef27f44c6aa029c376d49 and 3f492384a2f9b1126073b39a945eae222175deb7
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
........   (48/48)


module-items-font... FAILED
[ERROR] (line 3) Error: Evaluation failed: expected `"Fira Sans", Arial, sans-serif` for key `font-family` for selector `.item-table .module-item a`, found `"Fira Sans", Arial, NanumBarunGothic, sans-serif`: for command `assert-css: (".item-table .module-item a", {"font-family": '"Fira Sans", Arial, sans-serif'}, ALL)`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16
