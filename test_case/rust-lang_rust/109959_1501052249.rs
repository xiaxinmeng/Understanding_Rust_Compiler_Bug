diff
diff --git a/compiler/rustc_const_eval/src/transform/validate.rs b/compiler/rustc_const_eval/src/transform/validate.rs
index 0f56fda18f5..d4bed97380b 100644
--- a/compiler/rustc_const_eval/src/transform/validate.rs
+++ b/compiler/rustc_const_eval/src/transform/validate.rs
@@ -679,13 +679,21 @@ macro_rules! check_kinds {
                             // Unlike `mem::transmute`, a MIR `Transmute` is well-formed
                             // for any two `Sized` types, just potentially UB to run.
 
-                            if !op_ty.is_sized(self.tcx, self.param_env) {
+                            if !self
+                                .tcx
+                                .normalize_erasing_regions(self.param_env, op_ty)
+                                .is_sized(self.tcx, self.param_env)
+                            {
                                 self.fail(
                                     location,
                                     format!("Cannot transmute from non-`Sized` type {op_ty:?}"),
                                 );
                             }
-                            if !target_type.is_sized(self.tcx, self.param_env) {
+                            if !self
+                                .tcx
+                                .normalize_erasing_regions(self.param_env, *target_type)
+                                .is_sized(self.tcx, self.param_env)
+                            {
                                 self.fail(
                                     location,
                                     format!("Cannot transmute to non-`Sized` type {target_type:?}"),
