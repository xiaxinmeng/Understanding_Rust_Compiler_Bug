plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0d1754e8bf6942b4c1d24d7c923438782129ba5a and b42b4ba27d842fb5be99c85fbe73a3e4402fa7b5
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.          (51/51)


search-filter... FAILED
[ERROR] (line 5) "#results .externcrate" not found: for command `assert-text: ("#results .externcrate", "test_docs")`
search-result-color... FAILED
search-result-color... FAILED
[ERROR] (line 17) XPath "//*[@class='result-name']//*[text()='(keyword)']" not found: for command `// Checking the color for "keyword".
assert-css: ("//*[@class='result-name']//*[text()='(keyword)']", {"color": "rgb(120, 135, 151)"})`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16
