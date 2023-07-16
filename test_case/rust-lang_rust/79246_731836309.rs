patch
diff --git a/compiler/rustc_codegen_llvm/src/type_of.rs b/compiler/rustc_codegen_llvm/src/type_of.rs
index 8ea4768f77d..0876907e119 100644
--- a/compiler/rustc_codegen_llvm/src/type_of.rs
+++ b/compiler/rustc_codegen_llvm/src/type_of.rs
@@ -40,9 +40,7 @@ fn uncached_llvm_type<'a, 'tcx>(
         // FIXME(eddyb) producing readable type names for trait objects can result
         // in problematically distinct types due to HRTB and subtyping (see #47638).
         // ty::Dynamic(..) |
-        ty::Adt(..) | ty::Closure(..) | ty::Foreign(..) | ty::Generator(..) | ty::Str
-            if !cx.sess().fewer_names() =>
-        {
+        ty::Adt(..) | ty::Closure(..) | ty::Foreign(..) | ty::Generator(..) | ty::Str => {
             let mut name = with_no_trimmed_paths(|| layout.ty.to_string());
             if let (&ty::Adt(def, _), &Variants::Single { index }) =
                 (layout.ty.kind(), &layout.variants)
@@ -58,12 +56,6 @@ fn uncached_llvm_type<'a, 'tcx>(
             }
             Some(name)
         }
-        ty::Adt(..) => {
-            // If `Some` is returned then a named struct is created in LLVM. Name collisions are
-            // avoided by LLVM (with increasing suffixes). If rustc doesn't generate names then that
-            // can improve perf.
-            Some(String::new())
-        }
         _ => None,
     };
 
