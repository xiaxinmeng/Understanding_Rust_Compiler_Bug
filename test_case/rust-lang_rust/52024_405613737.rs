diff
diff --git a/src/librustc_typeck/check/mod.rs b/src/librustc_typeck/check/mod.rs
index 3e81262424..3a81891624 100644
--- a/src/librustc_typeck/check/mod.rs
+++ b/src/librustc_typeck/check/mod.rs
@@ -826,6 +826,13 @@ fn typeck_tables_of<'a, 'tcx>(tcx: TyCtxt<'a, 'tcx, 'tcx>,
     let id = tcx.hir.as_local_node_id(def_id).unwrap();
     let span = tcx.hir.span(id);

+    // For the base function, first run the well-formedness check.
+    // This is a hack -- there should just be a `check_well_formed`
+    // query that works for any def-id.
+    if let Node::NodeItem(_) = tcx.hir.get(id) {
+        ty::query::queries::check_item_well_formed::ensure(tcx, def_id);
+    }
+
     // Figure out what primary body this item has.
     let (body_id, fn_decl) = primary_body_of(tcx, id).unwrap_or_else(|| {
         span_bug!(span, "can't type-check body of {:?}", def_id);
