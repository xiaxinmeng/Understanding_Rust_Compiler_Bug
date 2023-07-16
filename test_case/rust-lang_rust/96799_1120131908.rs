diff
diff --git a/compiler/rustc_resolve/src/lib.rs b/compiler/rustc_resolve/src/lib.rs
index 62485beac47..f1900700946 100644
--- a/compiler/rustc_resolve/src/lib.rs
+++ b/compiler/rustc_resolve/src/lib.rs
@@ -1537,19 +1537,26 @@ fn macro_def(&self, mut ctxt: SyntaxContext) -> DefId {
     }
 
     /// Entry point to crate resolution.
-    pub fn resolve_crate(&mut self, krate: &Crate) {
+    pub fn resolve_crate(&mut self, krate: &Crate) -> Result<(), ErrorGuaranteed> {
         self.session.time("resolve_crate", || {
             self.session.time("finalize_imports", || ImportResolver { r: self }.finalize_imports());
             self.session.time("resolve_access_levels", || {
                 AccessLevelsVisitor::compute_access_levels(self, krate)
             });
             self.session.time("finalize_macro_resolutions", || self.finalize_macro_resolutions());
+            // If we can't find all imports, we run into cascading errors where all imported items are marked as unresolved.
+            // Avoid that by aborting early if we can't resolve all imports.
+            if let Some(reported) = self.session.has_errors() {
+                return Err(reported);
+            }
             self.session.time("late_resolve_crate", || self.late_resolve_crate(krate));
             self.session.time("resolve_main", || self.resolve_main());
             self.session.time("resolve_check_unused", || self.check_unused(krate));
             self.session.time("resolve_report_errors", || self.report_errors(krate));
             self.session.time("resolve_postprocess", || self.crate_loader.postprocess(krate));
-        });
+
+            Ok(())
+        })
     }
 
     pub fn traits_in_scope(
