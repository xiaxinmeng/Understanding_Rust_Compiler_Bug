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
Searching for toolstate changes between 7d6bf861f8c264d3d5a023e4a20c6007d1ee9018 and 89ead8edc7e0344d81377f1c84c6e83861eac484
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
basic-code... ok
basic... ok
check_info_sign_position... ok
code-blocks-overflow... FAILED
[ERROR] (line 6) ".docblock > .example-wrap > .language-txt" not found: for command `assert: ".docblock > .example-wrap > .language-txt"`
code-sidebar-toggle... ok
code_block_lines... ok
default-settings... ok
docblock-table-overflow... ok
---
trait-sidebar-item-order... ok
type-declation-overflow... ok


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:53
