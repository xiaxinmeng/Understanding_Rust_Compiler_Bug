plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              beta       -> FETCH_HEAD
Searching for toolstate changes between b4e8596e3e78395543747ebaba26b49f6a478aa8 and 724d6b14464d27d0ad2bb14f8a682dd29b87aac8
Submodules were updated
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
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
{"miri":"test-fail","rls":"build-fail","nomicon":"test-pass","rust-by-example":"test-pass","reference":"test-pass","edition-guide":"test-pass","embedded-book":"test-pass","rustbook":"test-fail","cargo-miri":"test-fail","book":"test-pass"}Build completed unsuccessfully in 0:00:00

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
proper steps.
