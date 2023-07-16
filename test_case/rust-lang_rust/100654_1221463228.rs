plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 903:
             obligation.cause.code()
         {
             &parent_code
-        } else if let ObligationCauseCode::ItemObligation(_) | ObligationCauseCode::ExprItemObligation(..) = obligation.cause.code() {
+        } else if let ObligationCauseCode::ItemObligation(_)
+        | ObligationCauseCode::ExprItemObligation(..) = obligation.cause.code()
             obligation.cause.code()
             obligation.cause.code()
         } else if let ExpnKind::Desugaring(DesugaringKind::ForLoop) =
             span.ctxt().outer_expn_data().kind
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 929:
         let param_env = obligation.param_env;
 
         // Try to apply the original trait binding obligation by borrowing.
-        let mut try_borrowing =
-            |old_pred: ty::PolyTraitPredicate<'tcx>, blacklist: &[DefId]| -> bool {
-                if blacklist.contains(&old_pred.def_id()) {
-                }
-                }
-                // We map bounds to `&T` and `&mut T`
-                let trait_pred_and_imm_ref = old_pred.map_bound(|trait_pred| {
-                        trait_pred,
-                        trait_pred,
-                        self.tcx.mk_imm_ref(self.tcx.lifetimes.re_static, trait_pred.self_ty()),
-                });
-                });
-                let trait_pred_and_mut_ref = old_pred.map_bound(|trait_pred| {
-                        trait_pred,
-                        trait_pred,
-                        self.tcx.mk_mut_ref(self.tcx.lifetimes.re_static, trait_pred.self_ty()),
-                });
-                });
+        let mut try_borrowing = |old_pred: ty::PolyTraitPredicate<'tcx>,
+                                 blacklist: &[DefId]|
+         -> bool {
+            if blacklist.contains(&old_pred.def_id()) {
+            }
+            }
+            // We map bounds to `&T` and `&mut T`
+            let trait_pred_and_imm_ref = old_pred.map_bound(|trait_pred| {
+                    trait_pred,
+                    trait_pred,
+                    self.tcx.mk_imm_ref(self.tcx.lifetimes.re_static, trait_pred.self_ty()),
+            });
+            });
+            let trait_pred_and_mut_ref = old_pred.map_bound(|trait_pred| {
+                    trait_pred,
+                    trait_pred,
+                    self.tcx.mk_mut_ref(self.tcx.lifetimes.re_static, trait_pred.self_ty()),
+            });
 
 
-                let mk_result = |trait_pred_and_new_ty| {
-                    let obligation =
-                        self.mk_trait_obligation_with_new_self_ty(param_env, trait_pred_and_new_ty);
-                    self.predicate_must_hold_modulo_regions(&obligation)
-                };
-                let imm_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_imm_ref);
-                let mut_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_mut_ref);
+            let mk_result = |trait_pred_and_new_ty| {
+                let obligation =
+                    self.mk_trait_obligation_with_new_self_ty(param_env, trait_pred_and_new_ty);
+                self.predicate_must_hold_modulo_regions(&obligation)
+            };
+            let imm_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_imm_ref);
+            let mut_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_mut_ref);
 
-                let (ref_inner_ty_satisfies_pred, ref_inner_ty_mut) =
+            let (ref_inner_ty_satisfies_pred, ref_inner_ty_mut) =
                 if let ObligationCauseCode::ItemObligation(_) | ObligationCauseCode::ExprItemObligation(..) = obligation.cause.code()
                     && let ty::Ref(_, ty, mutability) = old_pred.self_ty().skip_binder().kind()
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 968:
                     (false, false)
                 };
 
 
-                if imm_ref_self_ty_satisfies_pred
-                    || mut_ref_self_ty_satisfies_pred
-                    || ref_inner_ty_satisfies_pred
-                {
-                    if let Ok(snippet) = self.tcx.sess.source_map().span_to_snippet(span) {
-                        // We don't want a borrowing suggestion on the fields in structs,
-                        // struct Foo {
-                        // struct Foo {
-                        //  the_foos: Vec<Foo>
-                        // }
-                        // 