plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking chalk-engine v0.55.0
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error[E0501]: cannot borrow `*self` as mutable because previous closure requires unique access
     |
     |
1414 | / macro_rules! maybe_collect_tokens {
1415 | |     ($self:ident, $force_collect:expr, $attrs:expr, $f:expr) => {
1416 | |         if matches!($force_collect, ForceCollect::Yes)
1417 | |             || $crate::parser::attr::maybe_needs_tokens($attrs)
1418 | |         {
1419 | |             $self.collect_tokens_trailing_token($f)
     | |             |     |
     | |             |     |
     | |             |     first borrow later used by call
     | |             second borrow occurs here
1423 | |     };
1424 | | }
1424 | | }
     | |_- in this expansion of `maybe_collect_tokens!`
    ::: compiler/rustc_parse/src/parser/stmt.rs:96:60
     |
     |
96   |           maybe_collect_tokens!(self, force_collect, &attrs, |this: &mut Parser<'a>| {
     |           -                                                  ----------------------- closure construction occurs here
     | |
     | |
97   | |             let path = this.parse_path(PathStyle::Expr)?;
98   | |
99   | |             if this.eat(&token::Not) {
...    |
108  | |             let expr = if self.eat(&token::OpenDelim(token::Brace)) {
     | |                           ---- first borrow occurs due to use of `self` in closure
122  | |             ))
123  | |         })
     | |__________- in this macro invocation


error[E0501]: cannot borrow `*self` as mutable because previous closure requires unique access
     |
     |
1414 | /  macro_rules! maybe_collect_tokens {
1415 | |      ($self:ident, $force_collect:expr, $attrs:expr, $f:expr) => {
1416 | |          if matches!($force_collect, ForceCollect::Yes)
1417 | |              || $crate::parser::attr::maybe_needs_tokens($attrs)
...    |
1421 | |              Ok($f($self)?.0)
     | |                    ^^^^^ second borrow occurs here
1423 | |      };
1424 | |  }
1424 | |  }
     | |__- in this expansion of `maybe_collect_tokens!`
    ::: compiler/rustc_parse/src/parser/stmt.rs:96:60
     |
     |
96   |            maybe_collect_tokens!(self, force_collect, &attrs, |this: &mut Parser<'a>| {
     |            -                                                  -----------------------
     |            |                                                  |
     |   _________|__________________________________________________closure construction occurs here
     | ||
     | ||
97   | ||             let path = this.parse_path(PathStyle::Expr)?;
98   | ||
99   | ||             if this.eat(&token::Not) {
...    ||
108  | ||             let expr = if self.eat(&token::OpenDelim(token::Brace)) {
     | ||                           ---- first borrow occurs due to use of `self` in closure
122  | ||             ))
123  | ||         })
     | ||_________-- in this macro invocation
     |  |_________|
     |  |_________|
     |            first borrow later used by call

error[E0500]: closure requires unique access to `self` but it is already borrowed
     |
     |
96   |         maybe_collect_tokens!(self, force_collect, &attrs, |this: &mut Parser<'a>| {
     |                                                            ^^^^^^^^^^^^^^^^^^^^^^^ closure construction occurs here
...
108  |             let expr = if self.eat(&token::OpenDelim(token::Brace)) {
     |                           ---- second borrow occurs due to use of `self` in closure
    ::: compiler/rustc_parse/src/parser/mod.rs:1419:13
     |
     |
1419 |             $self.collect_tokens_trailing_token($f)
     |             ----- ----------------------------- first borrow later used by call
     |             |
     |             borrow occurs here
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0500, E0501.
For more information about an error, try `rustc --explain E0500`.
For more information about an error, try `rustc --explain E0500`.
error: could not compile `rustc_parse`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_serialize" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_feature" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_fs_util" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_incremental" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_mir_build" "-p" "rustc_ast_passes" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_save_analysis" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:23
