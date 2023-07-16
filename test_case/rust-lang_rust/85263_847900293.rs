plain

error[E0408]: variable `Wild` is not bound in all patterns
   --> compiler/rustc_mir_build/src/thir/visit.rs:194:9
    |
194 |         Binding { .. } | Wild => {}
    |         ^^^^^^^^^^^^^^   ---- variable not in all patterns
    |         |
    |         pattern doesn't bind `Wild`
    |
help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::Wild`
   --> compiler/rustc_mir_build/src/thir/visit.rs:194:26
    |
194 |         Binding { .. } | Wild => {}


error[E0422]: cannot find struct, variant or union type `AscribeUserType` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:191:9
    |
191 |         AscribeUserType { subpattern, .. }
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::AscribeUserType;
    |
1   | use rustc_infer::traits::query::type_op::AscribeUserType;
    |
1   | use rustc_middle::mir::StatementKind::AscribeUserType;
    |
1   | use rustc_middle::thir::PatKind::AscribeUserType;
      and 2 other candidates

error[E0422]: cannot find struct, variant or union type `Deref` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:192:11
   --> compiler/rustc_mir_build/src/thir/visit.rs:192:11
    |
192 |         | Deref { subpattern, .. }
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::ExprKind::Deref;
    |
1   | use crate::thir::visit::PatKind::Deref;
1   | use rustc_ast::UnOp::Deref;
    |
    |
1   | use rustc_hir::LangItem::Deref;
      and 6 other candidates


error[E0422]: cannot find struct, variant or union type `Binding` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:193:11
    |
193 |         | Binding { subpattern: Some(subpattern), .. } => visitor.visit_pat(&subpattern),
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::Binding;
1   | use rustc_hir::Node::Binding;
    |
    |
1   | use rustc_hir::OpaqueTyOrigin::Binding;
1   | use rustc_hir::PatKind::Binding;
    |
      and 1 other candidate


error[E0422]: cannot find struct, variant or union type `Binding` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:194:9
    |
194 |         Binding { .. } | Wild => {}
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::Binding;
1   | use rustc_hir::Node::Binding;
    |
    |
1   | use rustc_hir::OpaqueTyOrigin::Binding;
1   | use rustc_hir::PatKind::Binding;
    |
      and 1 other candidate


error[E0422]: cannot find struct, variant or union type `Variant` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:195:9
    |
195 |         Variant { subpatterns, .. } | Leaf { subpatterns } => {
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::Variant;
1   | use rustc_ast::Variant;
    |
1   | use rustc_hir::Node::Variant;
    |
    |
1   | use rustc_hir::Target::Variant;
    |
      and 4 other candidates

error[E0422]: cannot find struct, variant or union type `Leaf` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:195:39
    |
195 |         Variant { subpatterns, .. } | Leaf { subpatterns } => {
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::Leaf;
1   | use rustc_middle::mir::abstract_const::Node::Leaf;
    |
    |
1   | use rustc_middle::thir::PatKind::Leaf;
1   | use rustc_middle::ty::ValTree::Leaf;
    |

error[E0422]: cannot find struct, variant or union type `Constant` in this scope
error[E0422]: cannot find struct, variant or union type `Constant` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:200:9
    |
200 |         Constant { value } => visitor.visit_const(value),
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::Constant;
1   | use rustc_middle::mir::Constant;
    |
1   | use rustc_middle::mir::Operand::Constant;
    |
    |
1   | use rustc_middle::thir::PatKind::Constant;

error[E0531]: cannot find tuple struct or tuple variant `Range` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:201:9
    |
    |
201 |         Range(range) => {
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::Range;
1   | use rustc_ast::ExprKind::Range;
    |
1   | use rustc_ast::PatKind::Range;
    |
    |
1   | use rustc_hir::PatKind::Range;
    |
      and 1 other candidate

error[E0422]: cannot find struct, variant or union type `Slice` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:205:9
    |
205 |         Slice { prefix, slice, suffix } | Array { prefix, slice, suffix } => {
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::PatKind::Slice;
1   | use rustc_ast::PatKind::Slice;
    |
1   | use rustc_ast::TyKind::Slice;
    |
    |
1   | use rustc_hir::LangItem::Slice;
      and 5 other candidates


error[E0422]: cannot find struct, variant or union type `Array` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:205:43
    |
205 |         Slice { prefix, slice, suffix } | Array { prefix, slice, suffix } => {
    |
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::ExprKind::Array;
    |
1   | use crate::thir::visit::PatKind::Array;
1   | use rustc_ast::ExprKind::Array;
    |
    |
1   | use rustc_ast::ExprPrecedence::Array;
      and 12 other candidates


error[E0422]: cannot find struct, variant or union type `Or` in this scope
   --> compiler/rustc_mir_build/src/thir/visit.rs:216:9
    |
216 |         Or { pats } => {
    | 
   ::: /checkout/library/core/src/result.rs:245:5
    |
    |
245 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     --------------------------------------------------- similarly named variant `Ok` defined here
help: a variant with a similar name exists
    |
    |
216 |         Ok { pats } => {
help: consider importing one of these items
    |
    |
1   | use crate::thir::visit::LogicalOp::Or;
    |
1   | use crate::thir::visit::PatKind::Or;
1   | use rustc_ast::BinOpKind::Or;
    |
1   | use rustc_ast::PatKind::Or;
    |
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_symbol_mangling" "-p" "rustc_middle" "-p" "rustc_feature" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_data_structures" "-p" "rustc_span" "-p" "rustc_macros" "-p" "rustc_apfloat" "-p" "rustc_fs_util" "-p" "rustc_hir" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_error_codes" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:04:11
