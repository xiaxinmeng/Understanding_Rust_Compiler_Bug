plain
Step 5/10 : RUN npm install es-check -g
 ---> Running in 8a2466273c4c
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.3
added 95 packages from 44 contributors in 4.146s
Removing intermediate container 8a2466273c4c
 ---> c206399871f4
---
Successfully built d798827ff475
Successfully tagged rust-ci:latest
Built container sha256:d798827ff4755b030e9f76fe0f0a37b3e21f72342f51376af720e64326c72f6c
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
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
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:569:29
    |
569 |     tracked!(mir_opt_level, 3);
    |                             |
    |                             expected enum `Option`, found integer
    |                             expected enum `Option`, found integer
    |                             help: try using a variant of the expected enum: `Some(3)`
    = note: expected enum `Option<usize>`
    = note: expected enum `Option<usize>`
               found type `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_interface`
error: could not compile `rustc_interface`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_hir_pretty" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_data_structures" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_hir" "-p" "rustc_ast_pretty" "-p" "rustc_save_analysis" "-p" "rustc_serialize" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_incremental" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_symbol_mangling" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_ast" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:02:00
