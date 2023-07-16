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
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
313 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:314:23
    |
    |
314 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `framework` found for enum `NativeLibKind` in the current scope
    |
    |
319 |             kind: NativeLibKind::framework(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name (notice the capitalization): `Framework`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:320:23
    |
320 |             verbatim: false,
320 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:326:23
    |
326 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
335 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:336:23
    |
336 |             verbatim: false,
336 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `framework` found for enum `NativeLibKind` in the current scope
    |
    |
341 |             kind: NativeLibKind::framework(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name (notice the capitalization): `Framework`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:342:23
    |
342 |             verbatim: false,
342 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:348:23
    |
348 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
357 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:358:23
    |
358 |             verbatim: false,
358 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
363 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:364:23
    |
364 |             verbatim: false,
364 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:370:23
    |
370 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
379 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:380:23
    |
380 |             verbatim: false,
380 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `framework` found for enum `NativeLibKind` in the current scope
    |
    |
385 |             kind: NativeLibKind::framework(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name (notice the capitalization): `Framework`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:386:23
    |
386 |             verbatim: false,
386 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:392:23
    |
392 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
401 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:402:23
    |
402 |             verbatim: false,
402 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `framework` found for enum `NativeLibKind` in the current scope
    |
    |
407 |             kind: NativeLibKind::framework(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name (notice the capitalization): `Framework`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:408:23
    |
408 |             verbatim: true,
408 |             verbatim: true,
    |                       ^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(true)`
    = note: expected enum `Option<bool>`
               found type `bool`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:414:23
    |
414 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
435 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:436:23
    |
436 |             verbatim: false,
436 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `framework` found for enum `NativeLibKind` in the current scope
    |
    |
441 |             kind: NativeLibKind::framework(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name (notice the capitalization): `Framework`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:442:23
    |
442 |             verbatim: false,
442 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:448:23
    |
448 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `framework` found for enum `NativeLibKind` in the current scope
    |
    |
456 |             kind: NativeLibKind::framework(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name (notice the capitalization): `Framework`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:457:23
    |
457 |             verbatim: false,
457 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
462 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:463:23
    |
463 |             verbatim: false,
463 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:469:23
    |
469 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:478:23
    |
478 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `r#static` found for enum `NativeLibKind` in the current scope
    |
    |
483 |             kind: NativeLibKind::r#static(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name: `Static`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:484:23
    |
484 |             verbatim: false,
484 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`


error[E0599]: no variant or associated item named `framework` found for enum `NativeLibKind` in the current scope
    |
    |
489 |             kind: NativeLibKind::framework(),
    |                                  |
    |                                  |
    |                                  variant or associated item not found in `NativeLibKind`
    |                                  help: there is a variant with a similar name (notice the capitalization): `Framework`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:490:23
    |
490 |             verbatim: false,
490 |             verbatim: false,
    |                       ^^^^^
    |                       |
    |                       expected enum `Option`, found `bool`
    |                       help: try using a variant of the expected enum: `Some(false)`
    = note: expected enum `Option<bool>`
               found type `bool`

error: aborting due to 40 previous errors
error: aborting due to 40 previous errors

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_interface`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_lint_defs" "-p" "rustc_mir_build" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_errors" "-p" "rustc_ast_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_lint" "-p" "rustc_save_analysis" "-p" "rustc_target" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_resolve" "-p" "rustc_symbol_mangling" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "rustc_ast_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_plugin_impl" "-p" "rustc_serialize" "-p" "rustc_metadata" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:43
