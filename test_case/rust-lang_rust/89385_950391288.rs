plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 00d5e42e776da900049fe19087bc9b0057ec70cd and 81553a2960b7dad0c0b2591d548d87a59f0c641f
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
........   (48/48)


source-code-page... FAILED
[ERROR] (line 6) Error: Evaluation failed: expected `line-highlighted` for attribute `class` for selector `.line-numbers > span:nth-child(4)`, found `null`: for command `// Unfortunately, "#4" isn't a valid query selector, so we have to go around that limitation
// by instead getting the nth span.
assert-attribute: (".line-numbers > span:nth-child(4)", {"class": "line-highlighted"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:17
