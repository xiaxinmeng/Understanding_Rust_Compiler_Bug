plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ee5d8d37baaf5b5a81a98396952839c73ae41c68 and 0cf4c74da63ca680dc9e701d0b7f6a6dd371d06f
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/57)
.......    (57/57)


/checkout/src/test/rustdoc-gui/sidebar-mobile.goml sidebar-mobile... FAILED
[ERROR] line 24: `n`: booleans and idents cannot be used as keys


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:17
