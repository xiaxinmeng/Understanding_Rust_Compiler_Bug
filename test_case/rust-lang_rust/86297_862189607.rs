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
Searching for toolstate changes between 52b22869db9554728b0d2133a7c1dd9c713b3b16 and c6c019f8125d606936fa2cdea718d9506bfa6c1e
Executing the job since submodules are updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
error: tests/compile-fail/validity/invalid_enum_tag.rs:7: expected error not found: encountered 0x0000002a at .<enum-tag>, but expected a valid enum tag

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_enum_tag.rs" "-L" "/tmp/compiletest1t7cci" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest1t7cci/validity/invalid_enum_tag.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletest1t7cci/validity/invalid_enum_tag.stage-id.aux"
    Error {
        line_num: 7,
        kind: Some(
            Error,
---
error: tests/compile-fail/validity/invalid_enum_tag_256variants_uninit.rs:269: expected error not found: encountered uninitialized bytes at .<enum-tag>, but expected a valid enum tag

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_enum_tag_256variants_uninit.rs" "-L" "/tmp/compiletest1t7cci" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest1t7cci/validity/invalid_enum_tag_256variants_uninit.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletest1t7cci/validity/invalid_enum_tag_256variants_uninit.stage-id.aux"
    Error {
        line_num: 269,
        kind: Some(
            Error,
---
   2: compiletest_rs::runtest::run
   3: core::ops::function::FnOnce::call_once{{vtable.shim}}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/transmute_through_ptr.rs" "-L" "/tmp/compiletest1t7cci" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest1t7cci/validity/transmute_through_ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletest1t7cci/validity/transmute_through_ptr.stage-id.aux"
    Error {
        line_num: 13,
        kind: Some(
            Error,
---
Verifying status of rls...
Verifying status of miri...
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
proper steps.
{"rust-by-example":"test-pass","rls":"test-pass","rustbook":"test-fail","nomicon":"test-pass","reference":"test-pass","cargo-miri":"test-fail","edition-guide":"test-pass","embedded-book":"test-pass","miri":"test-fail","book":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
