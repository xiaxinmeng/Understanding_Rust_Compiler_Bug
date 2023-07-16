plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between bdcb52851231dc14bc6a7915dc62528cae7b8137 and bf8a9bd28cfe06f7788f01cd35bc6a01d1517a3b
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......    (47/47)


module-items-font... FAILED
[ERROR] (line 4) Error: Evaluation failed: expected `"Source Serif 4", "NanumBarunGothic", serif` for key `font-family` for selector `.item-table .docblock-short`, found `"Source Serif 4", NanumBarunGothic, serif`: for command `assert-css: (".item-table .docblock-short", {"font-family": '"Source Serif 4", "NanumBarunGothic", serif'}, ALL)`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:17
