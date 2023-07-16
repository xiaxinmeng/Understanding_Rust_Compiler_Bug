plain
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: unused import: `rustc_ast::AttrVec`
   |
17 | use rustc_ast::AttrVec;
   |     ^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `AttributesData`, `LazyTokenStream`, `PreexpTokenStream`, `PreexpTokenTree`, `Spacing`
   |
   |
10 |     AttributesData, DelimSpan, LazyTokenStream, PreexpTokenStream, PreexpTokenTree, Spacing,
   |     ^^^^^^^^^^^^^^             ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^

error: unused imports: `AttributesData`, `PreexpTokenStream`, `PreexpTokenTree`, `Spacing`
   |
   |
14 |     AttributesData, LazyTokenStream, PreexpTokenStream, PreexpTokenTree, Spacing,
   |     ^^^^^^^^^^^^^^                   ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling chalk-engine v0.36.0
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: aborting due to 3 previous errors
error: aborting due to 3 previous errors

error: could not compile `rustc_parse`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:10:19
