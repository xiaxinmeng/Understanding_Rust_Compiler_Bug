 diff
diff --git a/src/librustc_typeck/astconv.rs b/src/librustc_typeck/astconv.rs
index db9f6f5..dbb5ae9 100644
--- a/src/librustc_typeck/astconv.rs
+++ b/src/librustc_typeck/astconv.rs
@@ -1034,7 +1034,8 @@ pub fn ast_ty_to_ty<'tcx, AC: AstConv<'tcx>, RS: RegionScope>(
                         // FIXME(#19541): in both branches we should consider
                         // associated types in super-traits.
                         let (assoc_tys, tp_name): (Vec<_>, _) = match typ {
-                            def::TyParamProvenance::FromParam(did) => {
+                            def::TyParamProvenance::FromParam(did) |
+                            def::TyParamProvenance::FromSelf(did) => {
                                 let ty_param_defs = tcx.ty_param_defs.borrow();
                                 let tp_def = &(*ty_param_defs)[did.node];
                                 let assoc_tys = tp_def.bounds.trait_bounds.iter()
@@ -1042,13 +1043,6 @@ pub fn ast_ty_to_ty<'tcx, AC: AstConv<'tcx>, RS: RegionScope>(
                                     .collect();
                                 (assoc_tys, token::get_name(tp_def.name).to_string())
                             }
-                            def::TyParamProvenance::FromSelf(did) => {
-                                let assoc_tys = find_assoc_ty(this,
-                                                              &*this.get_trait_def(did).trait_ref,
-                                                              assoc_ident)
-                                    .into_iter().collect();
-                                (assoc_tys, "Self".to_string())
-                            }
                         };

                         if assoc_tys.len() == 0 {
