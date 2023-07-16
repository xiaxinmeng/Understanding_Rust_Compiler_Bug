diff
diff --git a/compiler/rustc_middle/src/ty/context/tls.rs b/compiler/rustc_middle/src/ty/context/tls.rs
index fb0d909307e..f2a93ea7e25 100644
--- a/compiler/rustc_middle/src/ty/context/tls.rs
+++ b/compiler/rustc_middle/src/ty/context/tls.rs
@@ -126,7 +126,7 @@ pub fn with_related_context<'tcx, F, R>(tcx: TyCtxt<'tcx>, f: F) -> R
         assert!(ptr::eq(
             context.tcx.gcx as *const _ as *const (),
             tcx.gcx as *const _ as *const ()
-        ));
+        ), "{:?} != {:?}", context.tcx.gcx as *const _, tcx.gcx as *const _);
 
         let context: &ImplicitCtxt<'_, '_> = unsafe { mem::transmute(context) };
 
