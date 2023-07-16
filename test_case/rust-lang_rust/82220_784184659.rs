plain
Removing intermediate container 587a53b3e661
 ---> f751d51eb8ba
Step 5/10 : RUN npm install es-check -g
 ---> Running in 403921a076e5
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.0
added 95 packages from 44 contributors in 4.572s
Removing intermediate container 403921a076e5
 ---> 85f1df963e8a
---
 ---> b97cbbeecb5f
Successfully built b97cbbeecb5f
Successfully tagged rust-ci:latest
Built container sha256:b97cbbeecb5ff54b8ff7859c3f4013e301988f8321c8c6c0fa657dd72d3260d9
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: the feature `if_let_guard` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
72 | #![feature(if_let_guard)]
   |
   |
   = note: `-D incomplete-features` implied by `-D warnings`

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to previous error


error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_target" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_privacy" "-p" "rustc_mir_build" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_hir_pretty" "-p" "rustc_feature" "-p" "rustc_error_codes" "-p" "rustc_ast_pretty" "-p" "rustc_session" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:48
