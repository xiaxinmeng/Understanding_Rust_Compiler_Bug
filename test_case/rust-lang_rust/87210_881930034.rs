plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f502bd3abd12111bbfae0974db018c165a977c0e and 872964b54689a387349f5a74f8fc729cd593d997
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
search-result-keyword... ok
search-tab-selection-if-current-is-empty... ok
shortcuts... ok
sidebar-mobile... FAILED
[ERROR] (line 17) Error: Evaluation failed: expected `0px` for key `left` for selector `.sidebar-elems`, found `-246px`: for command `assert-css: (".sidebar-elems", {"display": "block", "left": "0px"})`
sidebar... ok
source-code-page... ok
theme-change... ok
toggle-docs-mobile... ok
toggle-docs-mobile... ok
toggle-docs... ok
toggled-open-implementations... ok
trait-sidebar-item-order... ok
type-declation-overflow... ok


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:45
