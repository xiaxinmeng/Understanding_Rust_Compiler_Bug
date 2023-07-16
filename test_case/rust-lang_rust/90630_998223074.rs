plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 23f69235ad2eb9b44ac1a55eeaa3f9b484d9de4a and ef6d6f4e930f87f52b1902f6c5ad7ec7d462c0b5
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/54)
....       (54/54)


/checkout/src/test/rustdoc-gui/search-result-color.goml search-result-color... FAILED
[ERROR] (line 27) XPath "//*[@class='result-name']//*[text()='(keyword)']" not found: for command `assert-css: (
    "//*[@class='result-name']//*[text()='(keyword)']",
    {"color": "rgb(120, 135, 151)"},




command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:19
