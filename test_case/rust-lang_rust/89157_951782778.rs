plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 1b61b1b44581c449ae40505d181b00593f089d40 and 33e4df1626dee749ee2a6bb9b4f81a55b75c9d20
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
          (51/51)


search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 4) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `// Waiting for the search results to appear...
wait-for: "#titles"`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:43
