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
Searching for toolstate changes between 0f8cd43ee8c3614e04b5c624dd8a45758d7023da and e516676a845912510b5d8b57aad5b1981da922a6
Executing the job since clippy or rustfmt subtree was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

{"rustbook":"test-fail","edition-guide":"test-pass","miri":"build-fail","cargo-miri":"test-fail","reference":"test-pass","rls":"build-fail","nomicon":"test-pass","book":"test-pass","embedded-book":"test-pass","rust-by-example":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
Build completed unsuccessfully in 0:00:00
If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
proper steps.
