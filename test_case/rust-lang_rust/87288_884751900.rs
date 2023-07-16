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
Searching for toolstate changes between 7db08eeb0057de86ea2bdbd4c3a085cb8516b653 and 7edb3fed41b38576b3e49186017e21a3284eeda8
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
check_info_sign_position... ok
code-blocks-overflow... ok
code-sidebar-toggle... ok
default-settings... FAILED
[ERROR] (line 8) Error: Evaluation failed: expected `rgb(15, 20, 25)` for key `background-color` for selector `body`, found `rgb(255, 255, 255)`: for command `assert-css: ("body", {"background-color": "rgb(15, 20, 25)"})`
docblock-table-overflow... ok
escape-key... ok
font-weight... ok
hash-item-expansion... ok
---
trait-sidebar-item-order... ok
type-declation-overflow... ok


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:51
