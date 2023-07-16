diff
diff --git a/src/librustc_trans/debuginfo/mod.rs b/src/librustc_trans/debuginfo/mod.rs
index f05d48566d..e9468e5663 100644
--- a/src/librustc_trans/debuginfo/mod.rs
+++ b/src/librustc_trans/debuginfo/mod.rs
@@ -400,7 +400,7 @@ pub fn create_function_debug_context<'a, 'tcx>(cx: &CrateContext<'a, 'tcx>,
                 // Only "class" methods are generally understood by LLVM,
                 // so avoid methods on other types (e.g. `<*mut T>::null`).
                 match impl_self_ty.sty {
-                    ty::TyAdt(..) => {
+                    ty::TyAdt(def, ..) if !def.is_box() => {
                         Some(type_metadata(cx, impl_self_ty, syntax_pos::DUMMY_SP))
                     }
                     _ => None

