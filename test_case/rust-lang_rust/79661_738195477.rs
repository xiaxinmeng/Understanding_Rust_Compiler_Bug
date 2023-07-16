diff
diff --git a/src/lib.rs b/src/lib.rs
index ba9ee0d..82a88b1 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -204,11 +204,7 @@ fn codegen_crate<'tcx>(
         metadata: EncodedMetadata,
         need_metadata_module: bool,
     ) -> Box<dyn Any> {
-        let res = driver::codegen_crate(tcx, metadata, need_metadata_module, self.config);
-
-        rustc_symbol_mangling::test::report_symbol_names(tcx);
-
-        res
+        driver::codegen_crate(tcx, metadata, need_metadata_module, self.config)
     }
 
     fn join_codegen(
@@ -229,8 +225,6 @@ fn link(
     ) -> Result<(), ErrorReported> {
         use rustc_codegen_ssa::back::link::link_binary;
 
-        let _timer = sess.prof.generic_activity("link_crate");
-
         sess.time("linking", || {
             let target_cpu = crate::target_triple(sess).to_string();
             link_binary::<crate::archive::ArArchiveBuilder<'_>>(
