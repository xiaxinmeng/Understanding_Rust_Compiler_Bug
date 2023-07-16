plain
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
warning: rustc_main.4c8xs3mh-cgu.1: No profile data available for function _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hce97d5db31b8c6a0E Hash = 170957022131388415

warning: rustc_main.4c8xs3mh-cgu.0: No profile data available for function _ZN3std2rt10lang_start17h3704e418e905acf6E Hash = 742261418966908927

warning: rustc_main.4c8xs3mh-cgu.0: No profile data available for function _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf6d36b25f117822bE Hash = 742261418966908927

warning: rustc_main.4c8xs3mh-cgu.0: No profile data available for function _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h10d85659c65053d9E Hash = 742261418966908927

warning: rustc_main.4c8xs3mh-cgu.0: No profile data available for function _ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h4dd6af876446dabfE Hash = 742261418966908927

warning: rustc_main.4c8xs3mh-cgu.2: No profile data available for function _ZN10rustc_main4main17h947144eea8081511E Hash = 742261418966908927

warning: rustc_main.4c8xs3mh-cgu.2: No profile data available for function __rg_alloc Hash = 146835647075900052

warning: rustc_main.4c8xs3mh-cgu.2: No profile data available for function __rg_dealloc Hash = 742261418966908927

warning: rustc_main.4c8xs3mh-cgu.2: No profile data available for function __rg_realloc Hash = 146835647075900052

warning: rustc_main.4c8xs3mh-cgu.2: No profile data available for function __rg_alloc_zeroed Hash = 146835647075900052

warning: rustc_main.4c8xs3mh-cgu.2: No profile data available for function main Hash = 742261418966908927
warning: 11 warnings emitted

    Finished release [optimized] target(s) in 3m 41s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
   Compiling racer v2.1.45
   Compiling rustc-ap-rustc_expand v712.0.0
   Compiling rustfmt-nightly v1.4.37 (/checkout/src/tools/rustfmt)
    Finished release [optimized] target(s) in 1m 25s
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:196:13
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:


the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  rand_core 0.5.1 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {"getrandom", "alloc", "std"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librand_core-d024f93e5baa504a.rlib"
    `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librand_core-2bcb84d8da4d7a50.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
