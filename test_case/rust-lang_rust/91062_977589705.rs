plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between de4b242e1e2143f549f25ac5a8f7de9d902ef3b4 and eddbe84c38e27021f2b4fc5fce9e1b683ca60789
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (50/53)
...        (53/53)


/checkout/src/test/rustdoc-gui/toggle-click-deadspace.goml toggle-click-deadspace... FAILED
[ERROR] (line 8) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".impl-items .rustdoc-toggle", {"open": ""})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:18
