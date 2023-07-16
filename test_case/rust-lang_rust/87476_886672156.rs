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
 * branch              stable     -> FETCH_HEAD
Searching for toolstate changes between c8fb0b5be2c95d90036d78a4cffa483005c5daee and e8539fd6764a00cbe4d3447ee70bfd8d06c3286c
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
Successfully built 0628a7590b7d
Successfully tagged rust-ci:latest
Built container sha256:0628a7590b7d7712999912ad14e4811ff8097c3e78f43ca4c5a8bdcda20fa918
Uploading finished image to https://ci-caches.rust-lang.org/docker/f75a68e63650b25fb917d13e984be6be8c39a3bafb509b5582d37962767289b5e4eafa67efa10eb44f3bbd0b06321b2d4c8fb401dde8498e738b60556209a147
upload failed: - to s3://rust-lang-ci-sccache2/docker/f75a68e63650b25fb917d13e984be6be8c39a3bafb509b5582d37962767289b5e4eafa67efa10eb44f3bbd0b06321b2d4c8fb401dde8498e738b60556209a147 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.17s
basic-code... FAILED
[ERROR] line 3: expected only a CSS selector or an XPath in the tuple, found 2 elements
check_info_sign_position... FAILED
check_info_sign_position... FAILED
[ERROR] line 5: expected only a CSS selector or an XPath in the tuple, found 2 elements
code-sidebar-toggle... FAILED
[ERROR] line 6: expected only a CSS selector or an XPath in the tuple, found 2 elements
escape-key... FAILED
[ERROR] line 5: expected only a CSS selector or an XPath in the tuple, found 3 elements
hash-item-expansion... FAILED
[ERROR] line 4: expected only a CSS selector or an XPath in the tuple, found 3 elements
impl-default-expansion... FAILED
[ERROR] line 3: expected only a CSS selector or an XPath in the tuple, found 3 elements
search-input-mobile... ok
search-result-colors... FAILED
search-result-colors... FAILED
[ERROR] line 13: expected only a CSS selector or an XPath in the tuple, found 2 elements
search-result-description... FAILED
[ERROR] line 5: expected only a CSS selector or an XPath in the tuple, found 2 elements
search-result-display... FAILED
[ERROR] line 8: expected only a CSS selector or an XPath in the tuple, found 2 elements
search-result-keyword... FAILED
[ERROR] line 9: expected only a CSS selector or an XPath in the tuple, found 2 elements
search-tab-selection-if-current-is-empty... FAILED
[ERROR] line 5: expected only a CSS selector or an XPath in the tuple, found 3 elements
shortcuts... FAILED
[ERROR] line 11: expected only a CSS selector or an XPath in the tuple, found 2 elements
sidebar... FAILED
sidebar... FAILED
[ERROR] line 2: expected only a CSS selector or an XPath in the tuple, found 2 elements
source-code-page... FAILED
[ERROR] line 6: expected only a CSS selector or an XPath in the tuple, found 3 elements
theme-change... FAILED
[ERROR] line 6: expected only a CSS selector or an XPath in the tuple, found 2 elements
toggle-docs-mobile... FAILED
[ERROR] line 3: expected only a CSS selector or an XPath in the tuple, found 3 elements
toggle-docs... FAILED
[ERROR] line 2: expected only a CSS selector or an XPath in the tuple, found 3 elements
trait-sidebar-item-order... FAILED
[ERROR] line 2: expected only a CSS selector or an XPath in the tuple, found 2 elements


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui" "--tests-folder" "/checkout/src/test/rustdoc-gui"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:12
