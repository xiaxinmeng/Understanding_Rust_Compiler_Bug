
diff --git a/src/librustc/middle/ty.rs b/src/librustc/middle/ty.rs
index c51fba8..8b1100a 100644
--- a/src/librustc/middle/ty.rs
+++ b/src/librustc/middle/ty.rs
@@ -4179,7 +4179,7 @@ pub fn has_attr(tcx: ctxt, did: def_id, attr: &str) -> bool {
     } else {
         let mut ret = false;
         do csearch::get_item_attrs(tcx.cstore, did) |meta_items| {
-            ret = attr::contains_name(meta_items, attr);
+            ret = ret || attr::contains_name(meta_items, attr);
         }
         ret
     }
