plain
Successfully built 39310e2e91aa
Successfully tagged rust-ci:latest
Built container sha256:39310e2e91aaa80a4472c6a15d1dc751b275faad24698085f73b19bcf454abfe
Uploading finished image to https://ci-caches.rust-lang.org/docker/0eee8635c627ca79234c8671635d4c0d7df08a8d5fc20ba0d4a88482f9c5dddee647242a6f516f33086d21c088c4d12f823f40f1161e27d9b9e1fc9749916dc2
upload failed: - to s3://rust-lang-ci-sccache2/docker/0eee8635c627ca79234c8671635d4c0d7df08a8d5fc20ba0d4a88482f9c5dddee647242a6f516f33086d21c088c4d12f823f40f1161e27d9b9e1fc9749916dc2 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0423]: expected value, found struct variant `NativeLibKind::Framework`
    |
    |
316 |         (String::from("b"), None, NativeLibKind::Framework),
    | 
   ::: /checkout/compiler/rustc_session/src/utils.rs:32:5
    |
32  |     Framework {
32  |     Framework {
    |     --------- `NativeLibKind::Framework` defined here
help: use struct literal syntax instead
    |
    |
316 |         (String::from("b"), None, NativeLibKind::Framework { /* fields */ }),
help: consider importing one of these items instead
    |
    |
1   | use crate::passes::PathKind::Framework;
    |
1   | use rustc_session::search_paths::PathKind::Framework;


error[E0423]: expected value, found struct variant `NativeLibKind::Framework`
    |
    |
323 |         (String::from("X"), None, NativeLibKind::Framework),
    | 
   ::: /checkout/compiler/rustc_session/src/utils.rs:32:5
    |
32  |     Framework {
32  |     Framework {
    |     --------- `NativeLibKind::Framework` defined here
help: use struct literal syntax instead
    |
    |
323 |         (String::from("X"), None, NativeLibKind::Framework { /* fields */ }),
help: consider importing one of these items instead
    |
    |
1   | use crate::passes::PathKind::Framework;
    |
1   | use rustc_session::search_paths::PathKind::Framework;


error[E0423]: expected value, found struct variant `NativeLibKind::Framework`
    |
    |
337 |         (String::from("b"), Some(String::from("X")), NativeLibKind::Framework),
    | 
   ::: /checkout/compiler/rustc_session/src/utils.rs:32:5
    |
32  |     Framework {
32  |     Framework {
    |     --------- `NativeLibKind::Framework` defined here
help: use struct literal syntax instead
    |
    |
337 |         (String::from("b"), Some(String::from("X")), NativeLibKind::Framework { /* fields */ }),
help: consider importing one of these items instead
    |
    |
1   | use crate::passes::PathKind::Framework;
    |
1   | use rustc_session::search_paths::PathKind::Framework;


error[E0423]: expected value, found struct variant `NativeLibKind::Framework`
    |
    |
361 |         (String::from("b"), None, NativeLibKind::Framework),
    | 
   ::: /checkout/compiler/rustc_session/src/utils.rs:32:5
    |
32  |     Framework {
32  |     Framework {
    |     --------- `NativeLibKind::Framework` defined here
help: use struct literal syntax instead
    |
    |
361 |         (String::from("b"), None, NativeLibKind::Framework { /* fields */ }),
help: consider importing one of these items instead
    |
    |
1   | use crate::passes::PathKind::Framework;
    |
1   | use rustc_session::search_paths::PathKind::Framework;


error[E0423]: expected value, found struct variant `NativeLibKind::Framework`
    |
    |
366 |         (String::from("b"), None, NativeLibKind::Framework),
    | 
   ::: /checkout/compiler/rustc_session/src/utils.rs:32:5
    |
32  |     Framework {
32  |     Framework {
    |     --------- `NativeLibKind::Framework` defined here
help: use struct literal syntax instead
    |
    |
366 |         (String::from("b"), None, NativeLibKind::Framework { /* fields */ }),
help: consider importing one of these items instead
    |
    |
1   | use crate::passes::PathKind::Framework;
    |
1   | use rustc_session::search_paths::PathKind::Framework;


error[E0423]: expected value, found struct variant `NativeLibKind::Framework`
    |
    |
374 |         (String::from("b"), None, NativeLibKind::Framework),
    | 
   ::: /checkout/compiler/rustc_session/src/utils.rs:32:5
    |
32  |     Framework {
32  |     Framework {
    |     --------- `NativeLibKind::Framework` defined here
help: use struct literal syntax instead
    |
    |
374 |         (String::from("b"), None, NativeLibKind::Framework { /* fields */ }),
help: consider importing one of these items instead
    |
    |
1   | use crate::passes::PathKind::Framework;
    |
1   | use rustc_session::search_paths::PathKind::Framework;

    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
315 |         (String::from("a"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`

error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
322 |         (String::from("a"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`

error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
329 |         (String::from("a"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`

error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
330 |         (String::from("b"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`

error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
336 |         (String::from("a"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`

error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
360 |         (String::from("a"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`

error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
367 |         (String::from("a"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:372:9
    |
    |
372 |         (String::from("c"), None, NativeLibKind::Unspecified),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_session::utils::NativeLib`, found tuple
    = note: expected struct `rustc_session::utils::NativeLib`
    = note: expected struct `rustc_session::utils::NativeLib`
                found tuple `(std::string::String, Option<_>, NativeLibKind)`

error[E0599]: no variant or associated item named `StaticBundle` found for enum `NativeLibKind` in the current scope
    |
    |
373 |         (String::from("a"), None, NativeLibKind::StaticBundle),
    |                                                  ^^^^^^^^^^^^ variant or associated item not found in `NativeLibKind`
error: aborting due to 15 previous errors

Some errors have detailed explanations: E0308, E0423, E0599.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_interface`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_lexer" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_symbol_mangling" "-p" "rustc_data_structures" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_session" "-p" "rustc_apfloat" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_typeck" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_lint" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_mir_build" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:56
