plain

warning: unused bitwise operation that must be used
  --> /tmp/ctfe-stress-4.rs:64:30
   |
64 | expensive_static!(OPS: i32 = ((((10 >> 1) + 3) * 7) / 2 - 12) << 4; [4 16 16 16 16]);

warning: 3 warnings emitted

+ cp -pri ../src/tools/cargo /tmp/cargo
---
[RUSTC-TIMING] rustc_middle test:false 155.051
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
[RUSTC-TIMING] rustc_driver test:false 25.258
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
warning: rustc_main.9jntyg83-cgu.1: No profile data available for function _ZN3std2rt10lang_start17hdf33ff13e4ec54cfE Hash = 12884901887

warning: rustc_main.9jntyg83-cgu.2: No profile data available for function _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcddb088708a8c900E Hash = 281487861612543

warning: rustc_main.9jntyg83-cgu.1: No profile data available for function _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h01083035550ef7f3E Hash = 12884901887

warning: rustc_main.9jntyg83-cgu.1: No profile data available for function _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h24487d1575d92685E Hash = 12884901887

warning: rustc_main.9jntyg83-cgu.1: No profile data available for function _ZN4core3ptr13drop_in_place17hb0ed202a8bbc3590E Hash = 12884901887

warning: rustc_main.9jntyg83-cgu.0: No profile data available for function _ZN10rustc_main4main17hd58bc9a0215fadfdE Hash = 12884901887

warning: rustc_main.9jntyg83-cgu.0: No profile data available for function main Hash = 12884901887
warning: 7 warnings emitted

[RUSTC-TIMING] rustc_main test:false 0.465
    Finished release [optimized] target(s) in 3m 19s
---
 Documenting rustc_target v0.0.0 (/checkout/compiler/rustc_target)
 Documenting rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
[RUSTC-TIMING] rustc_parse_format test:false 0.192
[RUSTC-TIMING] rustc_feature test:false 0.383
error: unresolved link to `repr(transparent)`
   --> compiler/rustc_feature/src/active.rs:46:19
    |
46  |                   $(#[doc = $doc])*
...
...
95  | / declare_features! (
96  | |     // -------------------------------------------------------------------------
97  | |     // feature-group-start: internal feature gates
98  | |     // -------------------------------------------------------------------------
628 | |     // -------------------------------------------------------------------------
629 | | );
    | |__- in this macro invocation
    |
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            Allows #[repr(transparent)] on unions (RFC 2645).
                     ^^^^^^^^^^^^^^^^^
    = note: no item named `repr(transparent)` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `repr(transparent)`
   --> compiler/rustc_feature/src/active.rs:46:19
    |
46  |                   $(#[doc = $doc])*
...
...
95  | / declare_features! (
96  | |     // -------------------------------------------------------------------------
97  | |     // feature-group-start: internal feature gates
98  | |     // -------------------------------------------------------------------------
628 | |     // -------------------------------------------------------------------------
629 | | );
    | |__- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            Allows #[repr(transparent)] on unions (RFC 2645).
                     ^^^^^^^^^^^^^^^^^
    = note: no item named `repr(transparent)` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

error: could not document `rustc_feature`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_feature compiler/rustc_feature/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-900d11de835d34e6.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-3f54fec704da7e59.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.50.0-nightly
  (22a066cfc
  2020-12-24)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
[RUSTC-TIMING] serde test:false 2.752
[RUSTC-TIMING] rustc_target test:false 2.870
[RUSTC-TIMING] rustc_ast test:false 3.839
error: build failed
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_parse" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_ast_passes" "-p" "rustc_ast" "-p" "rustc_middle" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_fs_util" "-p" "rustc_traits" "-p" "rustc_data_structures" "-p" "rustc_lexer" "-p" "rustc_symbol_mangling" "-p" "rustc_driver" "-p" "rustc_llvm" "-p" "rustc_query_system" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_incremental" "-p" "rustc_save_analysis" "-p" "coverage_test_macros" "-p" "rustc_graphviz" "-p" "rustc_lint" "-p" "rustc_arena" "-p" "rustc_target" "-p" "rustc_builtin_macros" "-p" "rustc_lint_defs" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_serialize" "-p" "rustc_parse_format" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_hir_pretty" "-p" "rustc_span" "-p" "rustc_typeck" "-p" "rustc_session" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "-p" "rustc_resolve" "-p" "rustc_ast_pretty" "-p" "rustc_type_ir" "-p" "rustc_ast_lowering" "-p" "rustc_macros" "-p" "rustc_apfloat" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_passes" "-p" "rustc_feature" "-p" "rustc_ty_utils" "-p" "rustc_mir"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:06:15
