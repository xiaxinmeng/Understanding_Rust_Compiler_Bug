plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 27143a9094b55a00d5f440b05b0cb4233b300d33 and a1056acff03db079efc3d53ab80d3a349741bbea
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.          (51/51)


sidebar... FAILED
[ERROR] (line 7) ".sidebar-elems > .crate > ul > li > a.current" not found: for command `// We check that we have the crates list and that the "current" on is "test_docs".
assert-text: (".sidebar-elems > .crate > ul > li > a.current", "test_docs")`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:15
