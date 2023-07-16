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
escape-key... ok
font-weight... ok
hash-item-expansion... ok
impl-default-expansion... ok
label-next-to-symbol... FAILED
[ERROR] line 9: Unknown command "compare-elements-pos"
search-filter... ok
search-input-mobile... ok
search-result-colors... ok
search-result-description... ok
---
toggled-open-implementations... ok
trait-sidebar-item-order... ok


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui" "--tests-folder" "/checkout/src/test/rustdoc-gui"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:34
