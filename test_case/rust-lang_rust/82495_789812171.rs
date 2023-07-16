plain
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
Diff in /checkout/compiler/rustc_mir_build/src/build/misc.rs at line 3:
 
 use crate::build::Builder;
 
-use rustc_trait_selection::infer::InferCtxtExt;
-use rustc_middle::ty::{self, Ty};
 use rustc_middle::mir::*;
+use rustc_middle::ty::{self, Ty};
 use rustc_span::{Span, DUMMY_SP};
+use rustc_trait_selection::infer::InferCtxtExt;
 
 impl<'a, 'tcx> Builder<'a, 'tcx> {
     /// Adds a new temporary value of type `ty` storing the result of
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 144:
                     true_block,
                     destination,
                     destination,
-                    Constant { span: expr_span, user_ty: None, literal: ty::Const::from_bool(this.tcx, true) },
+                    Constant {
+                        span: expr_span,
+                        user_ty: None,
+                        literal: ty::Const::from_bool(this.tcx, true),
                 );
 
 
                 this.cfg.push_assign_constant(
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 151:
                     source_info,
                     destination,
                     destination,
-                    Constant { span: expr_span, user_ty: None, literal: ty::Const::from_bool(this.tcx, false) },
+                    Constant {
+                        span: expr_span,
+                        user_ty: None,
+                        literal: ty::Const::from_bool(this.tcx, false),
                 );
 
 
                 // Link up both branches:
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 242:
                     BorrowKind::Shared => unpack!(block = this.as_read_only_place(block, &arg)),
                     _ => unpack!(block = this.as_place(block, &arg)),
-                let borrow =
-                let borrow =
-                    Rvalue::Ref(this.tcx.lifetimes.re_erased, *borrow_kind, arg_place);
+                let borrow = Rvalue::Ref(this.tcx.lifetimes.re_erased, *borrow_kind, arg_place);
                 this.cfg.push_assign(block, source_info, destination, borrow);
                 block.unit()
             }
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 273:
                     })
                     .collect();
 
-                let field_names: Vec<_> = (0..adt_def.variants[*variant_index].fields.len()).map(Field::new).collect();
+                let field_names: Vec<_> =
+                    (0..adt_def.variants[*variant_index].fields.len()).map(Field::new).collect();
 
                 let fields: Vec<_> = if let Some(FruInfo { base, field_types }) = base {
                     let place_builder = unpack!(block = this.as_place_builder(block, &base));
Diff in /checkout/compiler/rustc_mir_build/src/build/block.rs at line 110:
                     let_scope_stack.push(remainder_scope);
 
                     // Declare the bindings, which may create a source scope.
-                    let remainder_span =
-                        remainder_scope.span(this.tcx, &this.region_scope_tree);
+                    let remainder_span = remainder_scope.span(this.tcx, &this.region_scope_tree);
                     let visibility_scope =
                     let visibility_scope =
                         Some(this.new_source_scope(remainder_span, LintLevel::Inherited, None));
Diff in /checkout/compiler/rustc_mir_build/src/build/scope.rs at line 526:
                 self.source_scopes[source_scope].local_data.as_ref().assert_crate_local().lint_root,
                 self.hir_id,
             );
-            let current_root =
-                tcx.maybe_lint_level_root_bounded(current_hir_id, self.hir_id);
+            let current_root = tcx.maybe_lint_level_root_bounded(current_hir_id, self.hir_id);
 
             if parent_root != current_root {
                 self.source_scope = self.new_source_scope(
Diff in /checkout/compiler/rustc_mir_build/src/build/scope.rs at line 834:
 
 
             if scope.region_scope == region_scope {
-                let region_scope_span =
-                    region_scope.span(self.tcx, &self.region_scope_tree);
+                let region_scope_span = region_scope.span(self.tcx, &self.region_scope_tree);
                 // Attribute scope exit drops to scope's closing brace.
                 let scope_end = self.tcx.sess.source_map().end_point(region_scope_span);
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 1:
 use crate::build;
 use crate::build;
 use crate::build::scope::DropKind;
 use crate::thir::cx::Cx;
-use crate::thir::{BindingMode, LintLevel, Expr, Pat, PatKind};
+use crate::thir::{BindingMode, Expr, LintLevel, Pat, PatKind};
 use rustc_attr::{self as attr, UnwindAttr};
 use rustc_errors::ErrorReported;
 use rustc_hir as hir;
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 14:
 use rustc_middle::mir::*;
 use rustc_middle::mir::*;
 use rustc_middle::ty::subst::Subst;
-use rustc_middle::ty::{self, Ty, TyCtxt, TypeckResults, TypeFoldable};
+use rustc_middle::ty::{self, Ty, TyCtxt, TypeFoldable, TypeckResults};
 use rustc_span::symbol::{kw, sym};
 use rustc_span::Span;
 use rustc_target::spec::abi::Abi;
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 692:
 ///
 /// This is required because we may still want to run MIR passes on an item
 /// with type errors, but normal MIR construction can't handle that in general.
-fn construct_error<'a, 'tcx>(infcx: &'a InferCtxt<'a, 'tcx>, def: ty::WithOptConstParam<LocalDefId>, hir_id: hir::HirId, body_id: hir::BodyId, body_owner_kind: hir::BodyOwnerKind) -> Body<'tcx> {
+fn construct_error<'a, 'tcx>(
+    infcx: &'a InferCtxt<'a, 'tcx>,
+    def: ty::WithOptConstParam<LocalDefId>,
+    hir_id: hir::HirId,
+    body_id: hir::BodyId,
+    body_owner_kind: hir::BodyOwnerKind,
+) -> Body<'tcx> {
     let tcx = infcx.tcx;
     let span = tcx.hir().span(hir_id);
     let ty = tcx.ty_error();
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 748:
         // Respect -C overflow-checks.
         check_overflow |= tcx.sess.overflow_checks();
         // Constants always need overflow checks.
-        check_overflow |= matches!(tcx.hir().body_owner_kind(hir_id), hir::BodyOwnerKind::Const | hir::BodyOwnerKind::Static(_));
+        check_overflow |= matches!(
+            tcx.hir().body_owner_kind(hir_id),
+            hir::BodyOwnerKind::Const | hir::BodyOwnerKind::Static(_)
+        );
 
         let lint_level = LintLevel::Explicit(hir_id);
         let mut builder = Builder {
Diff in /checkout/compiler/rustc_mir_build/src/build/matches/util.rs at line 15:
         subpatterns
             .iter()
             .map(|fieldpat| {
-                let place =
-                    self.tcx.mk_place_field(place, fieldpat.field, fieldpat.pattern.ty);
+                let place = self.tcx.mk_place_field(place, fieldpat.field, fieldpat.pattern.ty);
                 MatchPair::new(place, &fieldpat.pattern)
             .collect()
             .collect()
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs at line 462:
                             inferred_ty: expr.ty,
                         });
-                    let place =
-                    let place =
-                        place_builder.clone().into_place(this.tcx, this.typeck_results);
+                    let place = place_builder.clone().into_place(this.tcx, this.typeck_results);
                     this.cfg.push(
                         Statement {
                         Statement {
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs at line 616:
         if is_outermost_index {
             self.read_fake_borrows(block, fake_borrow_temps, source_info)
-            base_place =
-            base_place =
-                base_place.expect_upvars_resolved(self.tcx, self.typeck_results);
+            base_place = base_place.expect_upvars_resolved(self.tcx, self.typeck_results);
             self.add_fake_borrows_of_base(
                 &base_place,
                 block,
Diff in /checkout/compiler/rustc_mir_build/src/thir/cx/mod.rs at line 27:
 
 
 impl<'tcx> Cx<'tcx> {
-    crate fn new(
-        tcx: TyCtxt<'tcx>,
-        def: ty::WithOptConstParam<LocalDefId>,
-    ) -> Cx<'tcx> {
+    crate fn new(tcx: TyCtxt<'tcx>, def: ty::WithOptConstParam<LocalDefId>) -> Cx<'tcx> {
         let typeck_results = tcx.typeck_opt_const_arg(def);
         Cx {
             tcx,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast/src/util/literal.rs" "/checkout/compiler/rustc_ast/src/util/comments/tests.rs" "/checkout/compiler/rustc_mir_build/src/lints.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/compiler/rustc_mir_build/src/build/mod.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/compiler/rustc_mir_build/src/build/cfg.rs" "/checkout/compiler/rustc_ast/src/util/parser.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
