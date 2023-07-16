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
   Compiling rustc-ap-rustc_expand v691.0.0
   Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
   Compiling rustfmt-nightly v1.4.29 (/checkout/src/tools/rustfmt)
    Finished release [optimized] target(s) in 1m 28s
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:196:13
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  libz-sys 1.1.2 (registry+https://github.com/rust-lang/crates.io-index)
    `rustfmt` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibz_sys-24dbd2fa49524ada.rlib"
    `rls` additionally enabled features {"default", "stock-zlib"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibz_sys-ccddb11a0d4112cb.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
Build completed unsuccessfully in 0:29:06
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
error: Tool `rustfmt` was not recorded in tool state.
error: Tool `miri` was not recorded in tool state.
{"nomicon":"test-pass","rust-by-example":"test-pass","edition-guide":"test-pass","book":"test-pass","rls":"test-pass","embedded-book":"test-pass","reference":"test-pass","rustbook":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
