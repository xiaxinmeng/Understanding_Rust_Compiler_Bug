plain
    Checking tracing-tree v0.1.9
    Checking chalk-solve v0.55.0
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
 Documenting rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: public documentation for `note_expected_found` links to private item `Diagnostic::note_expected_found`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |           #[doc = $e]
...
...
216 | /     forward!(pub fn note_expected_found(
217 | |         &mut self,
218 | |         expected_label: &dyn fmt::Display,
219 | |         expected: DiagnosticStyledString,
220 | |         found_label: &dyn fmt::Display,
221 | |         found: DiagnosticStyledString,
222 | |     ) -> &mut Self);
    |
    |
    = note: `-D rustdoc::private-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            See [`Diagnostic::note_expected_found()`].
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without


error: public documentation for `note_unsuccessful_coercion` links to private item `Diagnostic::note_unsuccessful_coercion`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |           #[doc = $e]
...
...
234 | /     forward!(pub fn note_unsuccessful_coercion(
235 | |         &mut self,
236 | |         expected: DiagnosticStyledString,
237 | |         found: DiagnosticStyledString,
238 | |     ) -> &mut Self);
    |
    |
    = note: the link appears in this line:
            
            See [`Diagnostic::note_unsuccessful_coercion()`].
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without


error: public documentation for `span_note` links to private item `Diagnostic::span_note`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |           #[doc = $e]
...
...
241 | /     forward!(pub fn span_note<S: Into<MultiSpan>>(
243 | |         sp: S,
244 | |         msg: &str,
245 | |     ) -> &mut Self);
    | |____________________- in this macro invocation
    | |____________________- in this macro invocation
    |
    = note: the link appears in this line:
            
            See [`Diagnostic::span_note()`].
                 ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without


error: public documentation for `warn` links to private item `Diagnostic::warn`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |         #[doc = $e]
...
...
246 |     forward!(pub fn warn(&mut self, msg: &str) -> &mut Self);
    |
    |
    = note: the link appears in this line:
            
            See [`Diagnostic::warn()`].
                 ^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without


error: public documentation for `span_warn` links to private item `Diagnostic::span_warn`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |         #[doc = $e]
...
...
247 |     forward!(pub fn span_warn<S: Into<MultiSpan>>(&mut self, sp: S, msg: &str) -> &mut Self);
    |
    |
    = note: the link appears in this line:
            
            See [`Diagnostic::span_warn()`].
                 ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without


error: public documentation for `help` links to private item `Diagnostic::help`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |         #[doc = $e]
...
...
248 |     forward!(pub fn help(&mut self, msg: &str) -> &mut Self);
    |
    |
    = note: the link appears in this line:
            
            See [`Diagnostic::help()`].
                 ^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without


error: public documentation for `span_help` links to private item `Diagnostic::span_help`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |           #[doc = $e]
...
...
249 | /     forward!(pub fn span_help<S: Into<MultiSpan>>(
251 | |         sp: S,
252 | |         msg: &str,
253 | |     ) -> &mut Self);
    | |____________________- in this macro invocation
    | |____________________- in this macro invocation
    |
    = note: the link appears in this line:
            
            See [`Diagnostic::span_help()`].
                 ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without


error: public documentation for `set_primary_message` links to private item `Diagnostic::set_primary_message`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:37:9
    |
37  |         #[doc = $e]
...
...
373 |     forward!(pub fn set_primary_message<M: Into<String>>(&mut self, msg: M) -> &mut Self);
    |
    |
    = note: the link appears in this line:
            
            See [`Diagnostic::set_primary_message()`].
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this link resolves only because you passed `--document-private-items`, but will break without

error: aborting due to 8 previous errors

error: could not document `rustc_errors`
error: could not document `rustc_errors`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_errors compiler/rustc_errors/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern annotate_snippets=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libannotate_snippets-0ea0b9d3249bb9d4.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libatty-71c7b148fcf8fd6c.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-b1d46d2ac7b59a5e.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-7b863244f66f3fe2.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-e1634dc0d7ab121f.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-b9a24f71da7b43c7.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-bf85542d47cdbd04.rmeta --extern termcolor=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtermcolor-fe1bc8b0f37aa311.rmeta --extern termize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtermize-171f44c23ca91d59.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-27629d44cb623deb.rmeta --extern unicode_width=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_width-2192745c2c5f3cb4.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.52.0-nightly
  (cee392aca
  2021-03-16)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_parse_format" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_codegen_ssa" "-p" "rustc_ast" "-p" "rustc_trait_selection" "-p" "rustc_passes" "-p" "rustc_hir" "-p" "rustc_parse" "-p" "rustc_llvm" "-p" "rustc_serialize" "-p" "rustc_error_codes" "-p" "rustc_query_system" "-p" "rustc_ast_pretty" "-p" "rustc_mir" "-p" "rustc_apfloat" "-p" "rustc_graphviz" "-p" "rustc_fs_util" "-p" "rustc_metadata" "-p" "rustc_resolve" "-p" "rustc_driver" "-p" "rustc_feature" "-p" "rustc_lint_defs" "-p" "coverage_test_macros" "-p" "rustc_mir_build" "-p" "rustc_expand" "-p" "rustc_arena" "-p" "rustc_ty_utils" "-p" "rustc_span" "-p" "rustc_incremental" "-p" "rustc_type_ir" "-p" "rustc_symbol_mangling" "-p" "rustc_macros" "-p" "rustc_privacy" "-p" "rustc_infer" "-p" "rustc_index" "-p" "rustc_hir_pretty" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_save_analysis" "-p" "rustc_session" "-p" "rustc_query_impl" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_data_structures" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_interface" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_builtin_macros" "-p" "rustc_ast_passes" "-p" "rustc_errors"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:06:48
