plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5883b875631ca8406eb43aff8cf644ac14c99d77 and 25e70d9a73ec9f5e17b65cd7666df43cc9d9ad4f
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (50/54)
...       (54/54)


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 10) Error: No node found for selector: #crate-search: for command `click: "#crate-search"`

/checkout/src/test/rustdoc-gui/toggle-docs-mobile.goml toggle-docs-mobile... FAILED
[ERROR] (line 5) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".top-doc", {"open": ""})`

/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 4) TimeoutError: waiting for selector "#search > h1" failed: timeout 30000ms exceeded: for command `wait-for: "#search > h1" // The search element is empty before the first search `



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:41
