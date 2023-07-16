plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              beta       -> FETCH_HEAD
Searching for toolstate changes between 708d57e288d051a6238ed56039ffeac158e10e84 and d33deee9835e847aaefaa0c36b3dc7bb03436903
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/run-build-from-ci.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
.....      (45/45)


header-size... FAILED
[ERROR] (line 19) Error: Evaluation failed: expected `17.6px` for key `font-size` for selector `h4#top-doc-prose-sub-sub-heading`, found `16px`: for command `assert-css: ("h4#top-doc-prose-sub-sub-heading", {"font-size": "17.6px"})`
sidebar... FAILED
sidebar... FAILED
[ERROR] (line 10) Error: Evaluation failed: "Macros" !== "Structs": for command `assert-text: (".sidebar-elems > .items > ul > li:nth-child(2)", "Structs")`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:15
