plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 24a789b666f2c36d443cde48b5baef04e5b3c76d and 097d8694b70108c603710b7c733254b1c7790dc9
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
..         (42/42)


source-code-page... FAILED
[ERROR] (line 6) Error: Evaluation failed: expected `line-highlighted` for attribute `class` for selector `.line-numbers > span:nth-child(4)`, found `null`: for command `assert-attribute: (".line-numbers > span:nth-child(4)", {"class": "line-highlighted"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16
