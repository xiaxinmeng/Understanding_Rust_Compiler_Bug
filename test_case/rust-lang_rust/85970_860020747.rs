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
Searching for toolstate changes between b84ffce1aa806d1d53c2588fa59ecc00474b66ce and 98daa26dc313e70e940b2a51e12117436c9b4678
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
check_info_sign_position... ok
code-sidebar-toggle... ok
escape-key... ok
hash-item-expansion... FAILED
[ERROR] (line 7) Error: Evaluation failed: expected `flex` for key `display` for `#method\.borrow`, found `block`: for command `assert: ("#method\.borrow", {"display": "flex"})`
impl-default-expansion... ok
list_code_block... ok
search-input-mobile... ok
search-result-colors... ok
---
toggle-docs... ok
trait-sidebar-item-order... ok


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui" "--tests-folder" "/checkout/src/test/rustdoc-gui"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:32
