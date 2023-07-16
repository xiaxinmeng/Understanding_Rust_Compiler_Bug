plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d3f300477b89e70dd42379ba53c0e8ff74e9c694 and df0fd722ccd5ec69e2306a9c4eb1486d0e50a3d6
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/54)
....       (54/54)


/checkout/src/test/rustdoc-gui/anchors.goml anchors... FAILED
[ERROR] (line 19) "#top-doc-prose-title" not found: for command `assert-css: ("#top-doc-prose-title", {"color": "rgb(0, 0, 0)"})`

/checkout/src/test/rustdoc-gui/headings.goml headings... FAILED
[ERROR] (line 20) "h2#top-doc-prose-title" not found: for command `assert-css: ("h2#top-doc-prose-title", {"font-size": "20.8px"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:18
