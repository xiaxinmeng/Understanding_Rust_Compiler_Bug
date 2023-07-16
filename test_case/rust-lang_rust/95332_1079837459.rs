diff
diff --git a/compiler/rustc_const_eval/src/interpret/machine.rs b/compiler/rustc_const_eval/src/interpret/machine.rs
index 7d75c84d108..121121cdab2 100644
--- a/compiler/rustc_const_eval/src/interpret/machine.rs
+++ b/compiler/rustc_const_eval/src/interpret/machine.rs
@@ -422,7 +422,7 @@ fn force_int_for_alignment_check(_memory_extra: &Self::MemoryExtra) -> bool {
 
     #[inline(always)]
     fn enforce_validity(_ecx: &InterpCx<$mir, $tcx, Self>) -> bool {
-        false // for now, we don't enforce validity
+        true
     }
 
     #[inline(always)]
