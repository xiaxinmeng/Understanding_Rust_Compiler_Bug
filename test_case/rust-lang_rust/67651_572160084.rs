diff
@@ -1392,12 +1392,12 @@ fn type_of(tcx: TyCtxt<'_>, def_id: DefId) -> Ty<'_> {
         Node::Field(field) => icx.to_ty(&field.ty),

         Node::Expr(&hir::Expr { kind: hir::ExprKind::Closure(.., gen), .. }) => {
-            if gen.is_some() {
-                return tcx.typeck_tables_of(def_id).node_type(hir_id);
-            }
-
             let substs = InternalSubsts::identity_for_item(tcx, def_id);
-            tcx.mk_closure(def_id, substs)
+            if let Some(movability) = gen {
+                tcx.mk_generator(def_id, substs, movability)
+            } else {
+                tcx.mk_closure(def_id, substs)
+            }
         }

         Node::AnonConst(_) => {
