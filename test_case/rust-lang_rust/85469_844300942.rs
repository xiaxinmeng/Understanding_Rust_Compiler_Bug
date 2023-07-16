plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
   --> compiler/rustc_middle/src/ty/fold.rs:285:80
    |
285 |         value.fold_with(&mut RegionFolder::new(self, skipped_regions, &mut f)).into_ok()
    |
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
   --> compiler/rustc_middle/src/ty/fold.rs:585:44
    |
585 |             value.fold_with(&mut replacer).into_ok()
    |
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
   --> compiler/rustc_middle/src/ty/fold.rs:611:44
    |
611 |             value.fold_with(&mut replacer).into_ok()
    |
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
    --> compiler/rustc_middle/src/ty/fold.rs:1054:53
     |
1054 |     value.fold_with(&mut Shifter::new(tcx, amount)).into_ok()
     |
     = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
     = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
     = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
  --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:40:18
40 |                 .into_ok()
   |                  ^^^^^^^
   |
   = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
   = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
   = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
   --> compiler/rustc_middle/src/ty/subst.rs:434:37
    |
434 |         self.fold_with(&mut folder).into_ok()
    |
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
   --> compiler/rustc_middle/src/ty/util.rs:574:45
    |
574 |         let substs = substs.fold_with(self).into_ok();
    |
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
   --> compiler/rustc_middle/src/ty/util.rs:581:65
    |
581 |                     let expanded_ty = self.fold_ty(concrete_ty).into_ok();
    |
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
    --> compiler/rustc_middle/src/ty/util.rs:1076:33
     |
1076 |     val.fold_with(&mut visitor).into_ok()
     |
     = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
     = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
     = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
  --> compiler/rustc_middle/src/ty/erase_regions.rs:12:58
   |
12 |     ty.super_fold_with(&mut RegionEraserVisitor { tcx }).into_ok()
   |
   = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
   = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
   = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
  --> compiler/rustc_middle/src/ty/erase_regions.rs:28:78
   |
28 |         let value1 = value.fold_with(&mut RegionEraserVisitor { tcx: self }).into_ok();
   |
   = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
   = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
   = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unwrap_infallible': newly added
   --> compiler/rustc_middle/src/ty/instance.rs:592:62
    |
592 |                         &mut PolymorphizationFolder { tcx }).into_ok();
    |
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = note: see issue #61695 <https://github.com/rust-lang/rust/issues/61695> for more information
    = help: add `#![feature(unwrap_infallible)]` to the crate attributes to enable
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `rustc_middle`
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_index" "-p" "rustc_expand" "-p" "rustc_lexer" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_serialize" "-p" "rustc_mir" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_apfloat" "-p" "rustc_lint" "-p" "rustc_parse" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_ty_utils" "-p" "rustc_data_structures" "-p" "rustc_save_analysis" "-p" "rustc_span" "-p" "rustc_mir_build" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:57
