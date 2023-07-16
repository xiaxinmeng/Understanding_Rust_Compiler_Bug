plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between cfa4ac66c194046f631ce076c75516ecfdeb77ee and 24e7863332a0043d9f1edb2f2ac569727f8af106
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (50/56)
......     (56/56)


/checkout/src/test/rustdoc-gui/toggle-docs-mobile.goml toggle-docs-mobile... FAILED
[ERROR] (line 5) Error: Evaluation failed: assert didn't fail: for command `assert-attribute-false: (".top-doc", {"open": ""})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:19
