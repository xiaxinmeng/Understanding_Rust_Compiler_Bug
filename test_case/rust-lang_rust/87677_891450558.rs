plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e3b1c12becebd21d88ca9f4364d7db8d1d380c18 and 1fb1a941acf3718004b45cb15db1c77693f22d57
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Verifying status of rls...
Verifying status of miri...
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.
{"edition-guide":"test-pass","rustbook":"test-fail","rls":"test-pass","nomicon":"test-pass","embedded-book":"test-pass","rust-by-example":"test-pass","reference":"test-pass","cargo-miri":"test-fail","book":"test-pass","miri":"build-fail"}Build completed unsuccessfully in 0:00:00

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
proper steps.
