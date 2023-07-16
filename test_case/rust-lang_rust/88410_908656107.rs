plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 6cfa773583bb5123e630668f5bfe466716225546 and 8de5e345e43f540895b243824e333f1b3a2e928b
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (40/40)


font-weight... FAILED
[ERROR] (line 16) expected 1 elements, found 0: for command `assert-count: ("//*[@class='docblock type-decl']/pre[@class='trait']/code/a[@class='constant']/text()", 1)`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:14
