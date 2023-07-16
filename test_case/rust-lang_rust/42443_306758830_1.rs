diff
diff --git a/src/librustc/traits/error_reporting.rs b/src/librustc/traits/error_reporting.rs
index 7375338..c8e99c0 100644
--- a/src/librustc/traits/error_reporting.rs
+++ b/src/librustc/traits/error_reporting.rs
@@ -648,6 +648,10 @@ impl<'a, 'gcx, 'tcx> InferCtxt<'a, 'gcx, 'tcx> {
                             kind,
                             found_kind);
 
+                        err.span_label(
+                            obligation.cause.span,
+                            format!("the requirement to implement `{}` derives from here", kind));
+
                         let infer_tables = match self.tables {
                             InferTables::Interned(tables) =>
                                 Some(InferTablesRef::Interned(tables)),
@@ -656,6 +660,8 @@ impl<'a, 'gcx, 'tcx> InferCtxt<'a, 'gcx, 'tcx> {
                             InferTables::Missing => None,
                         };
 
+                        // Additional context information explaining why the closure only implements
+                        // a particular trait.
                         if let Some(tables) = infer_tables {
                             match tables.closure_kinds.get(&node_id) {
                                 Some(&(ty::ClosureKind::FnOnce, Some((span, name)))) => {
@@ -670,11 +676,6 @@ impl<'a, 'gcx, 'tcx> InferCtxt<'a, 'gcx, 'tcx> {
                                 },
                                 _ => {}
                             }
-                        } else {
-                            err.span_note(
-                                obligation.cause.span,
-                                &format!("the requirement to implement `{}` \
-                                          derives from here", kind));
                         }
 
                         err.emit();
