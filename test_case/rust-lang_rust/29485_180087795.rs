 diff
diff --git a/src/librustc_trans/trans/attributes.rs b/src/librustc_trans/trans/attributes.rs
index 28dfa4e..95c3941 100644
--- a/src/librustc_trans/trans/attributes.rs
+++ b/src/librustc_trans/trans/attributes.rs
@@ -265,7 +265,7 @@ pub fn from_fn_type<'a, 'tcx>(ccx: &CrateContext<'a, 'tcx>, fn_type: ty::Ty<'tcx
                 // on memory dependencies rather than pointer equality
                 let interior_unsafe = mt.ty.type_contents(ccx.tcx()).interior_unsafe();

-                if mt.mutbl == hir::MutMutable || !interior_unsafe {
+                if mt.mutbl != hir::MutMutable && !interior_unsafe {
                     attrs.arg(idx, llvm::Attribute::NoAlias);
                 }
