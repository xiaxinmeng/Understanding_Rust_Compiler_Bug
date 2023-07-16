plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0658]: use of unstable library feature 'bench_black_box'
 --> compiler/rustc_index/src/bit_set/tests.rs:4:5
4 | use std::hint::black_box;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
  = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
  = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   --> compiler/rustc_index/src/bit_set/tests.rs:373:9
    |
373 |         black_box(bs.insert(black_box(100u32)));
    |
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   --> compiler/rustc_index/src/bit_set/tests.rs:373:29
    |
373 |         black_box(bs.insert(black_box(100u32)));
    |
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   --> compiler/rustc_index/src/bit_set/tests.rs:381:9
    |
381 |         black_box(bs.remove(black_box(100u32)));
    |
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   --> compiler/rustc_index/src/bit_set/tests.rs:381:29
    |
381 |         black_box(bs.remove(black_box(100u32)));
    |
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   --> compiler/rustc_index/src/bit_set/tests.rs:389:34
    |
389 |         bs.iter().map(|b: usize| black_box(b)).for_each(drop);
    |
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   --> compiler/rustc_index/src/bit_set/tests.rs:398:22
    |
398 |         ba.intersect(black_box(&bb));
    |
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
    = help: add `#![feature(bench_black_box)]` to the crate attributes to enable
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `rustc_index`
error: could not compile `rustc_index`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_arena" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_target" "-p" "rustc_data_structures" "-p" "rustc_errors" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_metadata" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:53
