patch
diff --git a/compiler/rustc_mir_transform/src/remove_zsts.rs b/compiler/rustc_mir_transform/src/remove_zsts.rs
index 25e3c52132c..d93ffa38c69 100644
--- a/compiler/rustc_mir_transform/src/remove_zsts.rs
+++ b/compiler/rustc_mir_transform/src/remove_zsts.rs
@@ -9,6 +9,10 @@
 
 impl<'tcx> MirPass<'tcx> for RemoveZsts {
     fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
+        // Avoid query cycles (generators require optimized MIR for layout).
+        if tcx.type_of(body.source.def_id()).is_generator() {
+            return;
+        }
         let param_env = tcx.param_env(body.source.def_id());
         let (basic_blocks, local_decls) = body.basic_blocks_and_local_decls_mut();
         for block in basic_blocks.iter_mut() {
