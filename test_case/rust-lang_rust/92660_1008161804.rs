plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 488acf86a75c56d30b16822e953c505a9e4901a7 and 740e603e68750065ee4e5fbf4011011c9b1a0f3f
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/56)
......     (56/56)


/checkout/src/test/rustdoc-gui/module-items-font.goml module-items-font... FAILED
[ERROR] (line 24) "#structs + .item-table .item-left a" not found: for command `assert-css: (
    "#structs + .item-table .item-left a",
    {"font-family": '"Fira Sans", Arial, NanumBarunGothic, sans-serif'},


/checkout/src/test/rustdoc-gui/sidebar.goml sidebar... FAILED
[ERROR] (line 11) Error: Evaluation failed: "Types" !== "Structs": for command `assert-text: (".sidebar-elems .items > ul > li:nth-child(3)", "Structs")`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:15
