
--- a/compiler/rustc_typeck/src/variance/mod.rs
+++ b/compiler/rustc_typeck/src/variance/mod.rs
@@ -37,6 +37,12 @@ fn crate_variances(tcx: TyCtxt<'_>, (): ()) -> CrateVariancesMap<'_> {
 }

 fn variances_of(tcx: TyCtxt<'_>, item_def_id: DefId) -> &[ty::Variance] {
+
+    // Skip items with no generics - there's nothing to infer in them.
+    if tcx.generics_of(item_def_id).count() == 0 {
+        return &[];
+    }
+
     match tcx.def_kind(item_def_id) {
         DefKind::Fn
         | DefKind::AssocFn
