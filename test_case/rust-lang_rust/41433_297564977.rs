diff
diff --git a/src/librustc_typeck/check/_match.rs b/src/librustc_typeck/check/_match.rs
index 1086773041..450b0f95fc 100644
--- a/src/librustc_typeck/check/_match.rs
+++ b/src/librustc_typeck/check/_match.rs
@@ -660,8 +660,14 @@ impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
                                etc: bool) {
         let tcx = self.tcx;

+        let mut diagn_did = variant.did;
         let (substs, kind_name) = match adt_ty.sty {
-            ty::TyAdt(adt, substs) => (substs, adt.variant_descr()),
+            ty::TyAdt(adt, substs) => {
+                if !adt.is_enum() && variant.ctor_kind != CtorKind::Fictive {
+                    diagn_did = tcx.parent_def_id(variant.did).expect("no struct def id");
+                }
+                (substs, adt.variant_descr())
+            }
             _ => span_bug!(span, "struct pattern is not an ADT")
         };

@@ -700,12 +706,12 @@ impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
                             struct_span_err!(tcx.sess, span, E0026,
                                              "{} `{}` does not have a field named `{}`",
                                              kind_name,
-                                             tcx.item_path_str(variant.did),
+                                             tcx.item_path_str(diagn_did),
                                              field.name)
                                 .span_label(span,
                                             &format!("{} `{}` does not have field `{}`",
                                                      kind_name,
-                                                     tcx.item_path_str(variant.did),
+                                                     tcx.item_path_str(diagn_did),
                                                      field.name))
                                 .emit();
