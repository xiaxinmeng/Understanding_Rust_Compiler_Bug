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
Searching for toolstate changes between 5b638c1d3751b7ab31cac9739add516bdf39e10a and e0b1e1c8b7b6a9bd6a149935b208f09d3c099b36
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
search-result-keyword... ok
search-tab-selection-if-current-is-empty... ok
shortcuts... ok
sidebar... FAILED
[ERROR] (line 14) Error: Evaluation failed: "Type Definitions" !== "Keywords": for command `assert: (".sidebar-elems > .items > ul > li:nth-child(6)", "Keywords")`
source-code-page... ok
theme-change... ok
toggle-docs-mobile... ok
toggle-docs... ok
toggle-docs... ok
trait-sidebar-item-order... ok


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui" "--tests-folder" "/checkout/src/test/rustdoc-gui"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:33
