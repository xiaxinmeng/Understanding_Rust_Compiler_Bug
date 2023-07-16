plain
Step 5/10 : RUN npm install es-check -g
 ---> Running in d1760d9a9aea
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.3
added 95 packages from 44 contributors in 3.465s
Removing intermediate container d1760d9a9aea
 ---> 549fbdc83da2
---
Successfully built 4ef3cac8b376
Successfully tagged rust-ci:latest
Built container sha256:4ef3cac8b3768a801ee97c9d409699be7d3f509d4701f9b31b3e077048f0038b
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling generic-array v0.14.4
    Checking unicode-normalization v0.1.13
error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/lib.rs:385:26
    |
385 |     debug_assert!(cursor.prev() == '/' && cursor.peek() == '/');
    |                          ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/lib.rs:401:26
    |
401 |     debug_assert!(cursor.prev() == '/' && cursor.peek() == '*');
    |                          ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/lib.rs:439:38
    |
439 |     debug_assert!(is_id_start(cursor.prev()));
    |                                      ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
  --> compiler/rustc_lexer/src/literals.rs:55:33
   |
55 |     debug_assert!('0' <= cursor.prev() && cursor.prev() <= '9');
   |                                 ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
  --> compiler/rustc_lexer/src/literals.rs:55:50
   |
55 |     debug_assert!('0' <= cursor.prev() && cursor.prev() <= '9');
   |                                                  ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/literals.rs:160:26
    |
160 |     debug_assert!(cursor.prev() == 'e' || cursor.prev() == 'E');
    |                          ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/literals.rs:160:50
    |
160 |     debug_assert!(cursor.prev() == 'e' || cursor.prev() == 'E');
    |                                                  ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/literals.rs:168:26
    |
168 |     debug_assert!(cursor.prev() == '\'');
    |                          ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/literals.rs:214:26
    |
214 |     debug_assert!(cursor.prev() == '\'');
    |                          ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/literals.rs:257:26
    |
257 |     debug_assert!(cursor.prev() == '"');
    |                          ^^^^ method not found in `&mut cursor::Cursor<'_>`

error[E0599]: no method named `prev` found for mutable reference `&mut cursor::Cursor<'_>` in the current scope
   --> compiler/rustc_lexer/src/literals.rs:295:26
    |
295 |     debug_assert!(cursor.prev() == 'r');
    |                          ^^^^ method not found in `&mut cursor::Cursor<'_>`
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_lexer`
error: could not compile `rustc_lexer`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_feature" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_errors" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_query_impl" "-p" "rustc_symbol_mangling" "-p" "rustc_typeck" "-p" "rustc_incremental" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_lint" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:30
