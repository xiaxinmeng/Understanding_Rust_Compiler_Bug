plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 521734787ecf80ff12df7ca5998f7ec0b3b7b2c9 and 23178556858bde0306218c9969dbb3273285fbe0
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (40/41)
.          (41/41)


code_block_lines... FAILED
[ERROR] (line 8) Error: Evaluation failed: "1
3
4
5
6
6
7" !== "1
2
3
4": for command `assert-text: (".top-doc .line-number", "1\n2\n3\n4")`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:14
