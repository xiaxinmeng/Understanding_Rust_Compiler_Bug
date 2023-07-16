plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking tracing-tree v0.1.9
error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:63:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
63  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
210 | /     forward!(pub fn note_expected_found(
211 | |         &mut self,
212 | |         expected_label: &dyn fmt::Display,
213 | |         expected: DiagnosticStyledString,
214 | |         found_label: &dyn fmt::Display,
215 | |         found: DiagnosticStyledString,
216 | |     ) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:63:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
63  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
218 | /     forward!(pub fn note_expected_found_extra(
219 | |         &mut self,
220 | |         expected_label: &dyn fmt::Display,
221 | |         expected: DiagnosticStyledString,
...   |
225 | |         found_extra: &dyn fmt::Display,
226 | |     ) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:63:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
63  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
228 | /     forward!(pub fn note_unsuccessful_coercion(
229 | |         &mut self,
230 | |         expected: DiagnosticStyledString,
231 | |         found: DiagnosticStyledString,
232 | |     ) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:63:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
63  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
234 |       forward!(pub fn note(&mut self, msg: &str) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:81:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
81  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
235 | /     forward!(pub fn span_note<S: Into<MultiSpan>>(
237 | |         sp: S,
238 | |         msg: &str,
239 | |     ) -> &mut Self);
    | |____________________- in this macro invocation
    | |____________________- in this macro invocation

error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:63:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
63  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
240 |       forward!(pub fn warn(&mut self, msg: &str) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:81:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
81  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
241 |       forward!(pub fn span_warn<S: Into<MultiSpan>>(&mut self, sp: S, msg: &str) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:63:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
63  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
242 |       forward!(pub fn help(&mut self, msg: &str) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:81:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
81  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
243 | /     forward!(pub fn span_help<S: Into<MultiSpan>>(
245 | |         sp: S,
246 | |         msg: &str,
247 | |     ) -> &mut Self);
    | |____________________- in this macro invocation
    | |____________________- in this macro invocation

error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:81:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
81  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
367 |       forward!(pub fn set_primary_message<M: Into<String>>(&mut self, msg: M) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:81:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
81  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
368 |       forward!(pub fn set_span<S: Into<MultiSpan>>(&mut self, sp: S) -> &mut Self);


error[E0710]: an unknown tool name found in scoped lint: `rustdoc::private_intra_doc_links`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:63:17
41  | / macro_rules! forward {
41  | / macro_rules! forward {
42  | |     // Forward pattern for &self -> &Self
43  | |     (
44  | |         $(#[$attrs:meta])*
...   |
63  | |         #[allow(rustdoc::private_intra_doc_links)] // we always document with --document-private-items
...   |
86  | |     };
87  | | }
    | |_- in this expansion of `forward!`
    | |_- in this expansion of `forward!`
...
369 |       forward!(pub fn code(&mut self, s: DiagnosticId) -> &mut Self);

    Checking chalk-solve v0.55.0
error: aborting due to 12 previous errors


For more information about this error, try `rustc --explain E0710`.
error: could not compile `rustc_errors`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_apfloat" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_symbol_mangling" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_fs_util" "-p" "rustc_session" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_target" "-p" "rustc_serialize" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_save_analysis" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_error_codes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:10
