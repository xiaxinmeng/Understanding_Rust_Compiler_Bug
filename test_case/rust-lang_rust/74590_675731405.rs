diff
diff --git a/src/librustc_typeck/check/mod.rs b/src/librustc_typeck/check/mod.rs
index 824e81a974c..a45836f3f59 100644
--- a/src/librustc_typeck/check/mod.rs
+++ b/src/librustc_typeck/check/mod.rs
@@ -722,7 +722,9 @@ struct CheckItemTypesVisitor<'tcx> {
 
 impl ItemLikeVisitor<'tcx> for CheckItemTypesVisitor<'tcx> {
     fn visit_item(&mut self, i: &'tcx hir::Item<'tcx>) {
-        check_item_type(self.tcx, i);
+        self.tcx.sess.time("check_item_type", || {
+            check_item_type(self.tcx, i);
+        })
     }
     fn visit_trait_item(&mut self, _: &'tcx hir::TraitItem<'tcx>) {}
     fn visit_impl_item(&mut self, _: &'tcx hir::ImplItem<'tcx>) {}
