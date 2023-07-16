plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 2a6fb9a4c0e5ca7a81999065943b211c226fe9d8 and b40b204f6a06de6e9585ebdba26a1711135373e1
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
error: tests/compile-fail/function_calls/exported_symbol_wrong_type.rs:8: expected error not found: attempt to call an exported symbol that is not defined as a function

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/function_calls/exported_symbol_wrong_type.rs" "-L" "/tmp/compiletest6xLO9d" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest6xLO9d/function_calls/exported_symbol_wrong_type.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletest6xLO9d/function_calls/exported_symbol_wrong_type.stage-id.aux"
    Error {
        line_num: 8,
        kind: Some(
            Error,
---
.......... (40/40)
           (40/40)


module-item-font... FAILED
[ERROR] line 5: expected only a CSS selector or an XPath in the tuple, found 2 elements


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:11
