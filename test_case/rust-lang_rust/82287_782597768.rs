plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: this file contains an unclosed delimiter
    --> compiler/rustc_typeck/src/check/expr.rs:2169:3
     |
50   | impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
...
1364 |                 _ => {
1364 |                 _ => {
     |                      - this delimiter might not be properly closed...
1372 |             }
1372 |             }
     |             - ...as it matches this but it has different indentation
2169 | }
     |   ^


error: expected one of `=>`, `if`, or `|`, found `.`
     |
1383 |         )
1383 |         )
     |          - expected one of `=>`, `if`, or `|`
1384 |         .span_label(span, format!("missing {}{}", remaining_fields_names, truncated_fields_error))
     |         ^ unexpected token

error: expected one of `.`, `;`, `?`, or an operator, found doc comment `/// Report an error for a struct field expression when there are no visible fields.`
     |
1386 |     }
1386 |     }
     |      - expected one of `.`, `;`, `?`, or an operator
1387 | 
1388 |     /// Report an error for a struct field expression when there are no visible fields.


error[E0425]: cannot find function `ty_kind_suggestion` in this scope
    |
    |
605 | ...                   if let Some(val) = ty_kind_suggestion(ty) {


error[E0425]: cannot find function `ty_kind_suggestion` in module `expr`
    --> compiler/rustc_typeck/src/check/mod.rs:1007:29
     |
1007 |             let val = expr::ty_kind_suggestion(ty).unwrap_or("value");
     |                             ^^^^^^^^^^^^^^^^^^ not found in `expr`

error: unused import: `YieldExprOutsideOfGenerator`
   |
   |
20 |     YieldExprOutsideOfGenerator,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: unused import: `rustc_ast as ast`
   |
25 | use rustc_ast as ast;
   |     ^^^^^^^^^^^^^^^^


error: unused imports: `pluralize`, `struct_span_err`
   |
   |
29 | use rustc_errors::{pluralize, struct_span_err, Applicability, DiagnosticBuilder, DiagnosticId};
   |                    ^^^^^^^^^  ^^^^^^^^^^^^^^^

error: unused import: `rustc_hir::def_id::DefId`
   |
32 | use rustc_hir::def_id::DefId;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^


error: unused import: `rustc_middle::ty::subst::SubstsRef`
   |
   |
39 | use rustc_middle::ty::subst::SubstsRef;

error: unused import: `Visibility`
  --> compiler/rustc_typeck/src/check/expr.rs:42:33
   |
   |
42 | use rustc_middle::ty::{AdtKind, Visibility};

error: unused import: `rustc_span::edition::LATEST_STABLE_EDITION`
  --> compiler/rustc_typeck/src/check/expr.rs:43:5
   |
---

error: unused import: `Symbol`
  --> compiler/rustc_typeck/src/check/expr.rs:47:42
   |
47 | use rustc_span::symbol::{kw, sym, Ident, Symbol};

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0599]: no method named `check_expr_asm` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
    |
    |
247 |             ExprKind::InlineAsm(asm) => self.check_expr_asm(asm),
    |                                              ^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `check_expr_cast`

error[E0599]: no method named `check_field` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
    |
    |
299 |             ExprKind::Field(base, field) => self.check_field(expr, &base, field),
    |                                                  ^^^^^^^^^^^ method not found in `&fn_ctxt::FnCtxt<'a, 'tcx>`

error[E0599]: no method named `check_expr_index` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
    |
    |
300 |             ExprKind::Index(base, idx) => self.check_expr_index(base, idx, expr),
    |                                                ^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `check_expr_kind`

error[E0599]: no method named `check_expr_yield` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
    |
    |
301 |             ExprKind::Yield(value, ref src) => self.check_expr_yield(value, expr, src),
    |                                                     ^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `check_expr_kind`

error[E0599]: no method named `report_unknown_field` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
     |
     |
1287 |                     self.report_unknown_field(adt_ty, variant, field, ast_fields, kind_name, span);
     |                          ^^^^^^^^^^^^^^^^^^^^ method not found in `&fn_ctxt::FnCtxt<'a, 'tcx>`

error[E0599]: no method named `report_no_accessible_fields` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
     |
     |
1312 |                 self.report_no_accessible_fields(adt_ty, span);
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&fn_ctxt::FnCtxt<'a, 'tcx>`
error: aborting due to 20 previous errors

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_errors" "-p" "rustc_macros" "-p" "rustc_lint_defs" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_data_structures" "-p" "rustc_target" "-p" "rustc_hir" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_save_analysis" "-p" "rustc_hir_pretty" "-p" "rustc_serialize" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_feature" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_interface" "-p" "rustc_mir_build" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:01
