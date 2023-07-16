plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f8751436ffce35cd1b7291b03b394166b77ff0da and 668f6d7f1929e85a08f7b1033fb5f085ee222221
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
...        (43/43)


search-result-go-to-first... FAILED
[ERROR] (line 20) Error: Evaluation failed: "Struct test_docs::Foo
        " !== "Struct test_docs::Foo": for command `assert-text: (".fqn .in-band", "Struct test_docs::Foo")`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16
