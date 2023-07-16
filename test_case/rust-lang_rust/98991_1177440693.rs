patch
diff --git a/src/librustc/middle/stability.rs b/src/librustc/middle/stability.rs
index f5e18e13465..084b3e19c74 100644
--- a/src/librustc/middle/stability.rs
+++ b/src/librustc/middle/stability.rs
@@ -558,8 +558,7 @@ impl<'a, 'tcx> Visitor<'tcx> for Checker<'a, 'tcx> {
                         let trait_item_def_id = self.tcx.associated_items(trait_did)
                             .find(|item| item.name == impl_item.name).map(|item| item.def_id);
                         if let Some(def_id) = trait_item_def_id {
-                            // Pass `DUMMY_NODE_ID` to skip deprecation warnings.
-                            self.tcx.check_stability(def_id, ast::DUMMY_NODE_ID, impl_item.span);
+                            self.tcx.check_stability(def_id, impl_item.id, impl_item.span);
                         }
                     }
                 }
