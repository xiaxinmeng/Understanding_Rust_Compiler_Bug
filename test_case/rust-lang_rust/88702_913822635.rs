plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 8ceea01bb442b9746a51b062ce25abbf46d866b2 and 296b14d72e4fbbe49de6a04bb62efaa44f677f51
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.          (41/41)


source-code-page... FAILED
[ERROR] (line 6) Error: Evaluation failed: expected `line-highlighted` for attribute `class` for selector `.line-numbers > span:nth-child(4)`, found `null`: for command `assert-attribute: (".line-numbers > span:nth-child(4)", {"class": "line-highlighted"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:13
