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
   Compiling rls-rustc v0.6.0 (/checkout/src/tools/rls/rls-rustc)
   Compiling rustc-ap-rustc_ast v712.0.0
   Compiling rustc-ap-rustc_target v712.0.0
   Compiling rustc-ap-rustc_feature v712.0.0
error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_ast-712.0.0/src/ptr.rs:136:37
    |
136 |                 std::mem::transmute(NonNull::<[T; 0]>::dangling() as NonNull<[T]>)
    |
    = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
    = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable

---
   Compiling rustc-ap-rustc_ast v712.0.0
   Compiling rustc-ap-rustc_target v712.0.0
   Compiling rustc-ap-rustc_feature v712.0.0
   Compiling openssl v0.10.30
error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_ast-712.0.0/src/ptr.rs:136:37
    |
136 |                 std::mem::transmute(NonNull::<[T; 0]>::dangling() as NonNull<[T]>)
    |
    = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
    = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable

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
{"reference":"test-pass","book":"test-pass","embedded-book":"test-pass","nomicon":"test-pass","edition-guide":"test-pass","miri":"test-pass","rustfmt":"build-fail","rls":"build-fail","rustbook":"test-fail","rust-by-example":"test-pass","cargo-miri":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
