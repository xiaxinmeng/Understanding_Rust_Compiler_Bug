
diff --git a/compiler/rustc_middle/src/ty/util.rs b/compiler/rustc_middle/src/ty/util.rs
index e23c3f51967..40ae83fad6d 100644
--- a/compiler/rustc_middle/src/ty/util.rs
+++ b/compiler/rustc_middle/src/ty/util.rs
@@ -222,6 +222,7 @@ pub fn struct_tail_with_normalize(
         normalize: impl Fn(Ty<'tcx>) -> Ty<'tcx>,
     ) -> Ty<'tcx> {
         loop {
+            let prev_ty = ty;
             match *ty.kind() {
                 ty::Adt(def, substs) => {
                     if !def.is_struct() {
@@ -254,6 +255,10 @@ pub fn struct_tail_with_normalize(
                     break;
                 }
             }
+
+            if prev_ty == ty {
+                panic!("Called struct_tail_with_normalize on recursive type: {:?}", ty);
+            }
         }
         ty
     }
