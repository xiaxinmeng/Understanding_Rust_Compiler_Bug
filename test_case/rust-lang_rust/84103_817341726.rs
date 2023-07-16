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
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
    | 
   ::: /checkout/library/core/src/default.rs:167:1
    |
167 | / pub macro Default($item:item) {
169 | | }
    | |_- in this expansion of `#[derive(Default)]`
    |
    = note: cannot satisfy `_: std::default::Default`
---
    |
note: function defined here
   --> /checkout/src/tools/cargo/src/cargo/ops/resolve.rs:206:8
    |
206 | pub fn resolve_with_previous<'cfg>(

error: aborting due to 11 previous errors

Some errors have detailed explanations: E0061, E0271, E0283, E0432, E0433, E0560.
---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
proper steps.
{"rust-by-example":"test-pass","cargo-miri":"test-fail","rls":"build-fail","rustbook":"test-fail","book":"test-pass","reference":"test-pass","nomicon":"test-pass","miri":"test-pass","embedded-book":"test-pass","edition-guide":"test-pass","rustfmt":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
