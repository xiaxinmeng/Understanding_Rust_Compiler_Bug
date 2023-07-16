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
   Compiling rustc-ap-rustc_arena v697.0.0
error[E0658]: const generics are unstable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_arena-697.0.0/src/lib.rs:136:15
    |
136 | impl<T, const N: usize> IterExt<T> for std::array::IntoIter<T, N> {
    |
    = note: see issue #74878 <https://github.com/rust-lang/rust/issues/74878> for more information
    = note: see issue #74878 <https://github.com/rust-lang/rust/issues/74878> for more information
    = help: add `#![feature(min_const_generics)]` to the crate attributes to enable
   Compiling proc-macro-error-attr v1.0.4
   Compiling proc-macro-error v1.0.4
error: aborting due to previous error

---
   Compiling heck v0.3.1
error[E0658]: const generics are unstable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_arena-697.0.0/src/lib.rs:136:15
    |
136 | impl<T, const N: usize> IterExt<T> for std::array::IntoIter<T, N> {
    |
    = note: see issue #74878 <https://github.com/rust-lang/rust/issues/74878> for more information
    = note: see issue #74878 <https://github.com/rust-lang/rust/issues/74878> for more information
    = help: add `#![feature(min_const_generics)]` to the crate attributes to enable
   Compiling byteorder v1.3.4
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
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
{"rls":"build-fail","reference":"test-pass","cargo-miri":"test-fail","rustbook":"test-fail","edition-guide":"test-pass","nomicon":"test-pass","embedded-book":"test-pass","rustfmt":"build-fail","rust-by-example":"test-pass","book":"test-pass","miri":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
