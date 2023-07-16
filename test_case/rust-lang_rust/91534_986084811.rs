plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between efec545293b9263be9edfb283a7aa66350b3acbf and 54950fa099c2535add36190c17d76ce65a8a48e2
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/53)
...        (53/53)


/checkout/src/test/rustdoc-gui/headers-color.goml headers-color... FAILED
[ERROR] (line 21) ".section-header a" not found: for command `assert-css: (".section-header a", {"color": "rgb(197, 197, 197)"}, ALL)`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:18
