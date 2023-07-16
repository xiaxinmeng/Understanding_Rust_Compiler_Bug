patch
index 461b13c4f63..d49014dc7cd 100644
--- a/src/librustc_mir/transform/generator.rs
+++ b/src/librustc_mir/transform/generator.rs
@@ -301,10 +301,13 @@ impl MutVisitor<'tcx> for TransformVisitor<'tcx> {
     }

     fn visit_basic_block_data(&mut self, block: BasicBlock, data: &mut BasicBlockData<'tcx>) {
-        // Remove StorageLive and StorageDead statements for remapped locals      
+        // Remove StorageLive and StorageDead statements for remapped locals.     
+        // Also remove storage dead from unwind paths where it's no longer        
+        // needed.
+        let is_cleanup = data.is_cleanup;
         data.retain_statements(|s| match s.kind {
             StatementKind::StorageLive(l) | StatementKind::StorageDead(l) => {    
-                !self.remap.contains_key(&l)
+                !is_cleanup && !self.remap.contains_key(&l)
             }
             _ => true,
         });
