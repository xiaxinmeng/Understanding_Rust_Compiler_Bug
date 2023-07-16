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
Searching for toolstate changes between ed597e7e19d0fe716d9f81b1e840a5abbfd7c28d and ca2f74a6a8dec84d0362621dc0702ed6010b18ca
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
sidebar... ok
source-code-page... ok
theme-change... ok
toggle-docs-mobile... FAILED
[ERROR] (line 5) assert didn't fail: for command `assert-false: (".top-doc", "open", "")`
toggle-docs... ok
trait-sidebar-item-order... ok



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui" "--tests-folder" "/checkout/src/test/rustdoc-gui"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:36
