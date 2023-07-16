plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e4b1d5841494d6eb7f4944c91a057e16b0f0a9ea and 56eb404f93248bc2774bb1e682d78381ab5f0d0b
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (50/56)
......     (56/56)


/checkout/src/test/rustdoc-gui/implementors.goml implementors... FAILED
[ERROR] (line 9) Error: Evaluation failed: expected `impl-Whatever` for attribute `id` for selector `#implementors-list .impl:nth-child(1)`, found `impl-Whatever-for-Struct`: for command `assert-attribute: ("#implementors-list .impl:nth-child(1)", {"id": "impl-Whatever"})`

/checkout/src/test/rustdoc-gui/toggle-click-deadspace.goml toggle-click-deadspace... FAILED
[ERROR] (line 11) Error: No node found for selector: #impl-Trait h3 a:first-of-type: for command `click: "#impl-Trait h3 a:first-of-type"`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:18
