plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 936ce3dab7fd042101767c439362310f8355e859 and ba271b298ab9dce96f704960d5bc8ca1e483bc42
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (50/54)
....       (54/54)


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 10) Error: No node found for selector: #crate-search: for command `click: "#crate-search"`

/checkout/src/test/rustdoc-gui/toggle-docs-mobile.goml toggle-docs-mobile... FAILED
[ERROR] (line 5) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".top-doc", {"open": ""})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:19
