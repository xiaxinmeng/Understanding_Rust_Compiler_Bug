diff
diff --git a/src/librustc_privacy/lib.rs b/src/librustc_privacy/lib.rs
index fc00050f405..5855b48c01d 100644
--- a/src/librustc_privacy/lib.rs
+++ b/src/librustc_privacy/lib.rs
@@ -782,8 +782,8 @@ impl Visitor<'tcx> for EmbargoVisitor<'tcx> {
                 // FIXME: This is some serious pessimization intended to workaround deficiencies
                 // in the reachability pass (`middle/reachable.rs`). Types are marked as link-time
                 // reachable if they are returned via `impl Trait`, even from private functions.
-                let exist_level = cmp::max(item_level, Some(AccessLevel::ReachableFromImplTrait));
-                self.reach(item.hir_id, exist_level).generics().predicates().ty();
+                //let exist_level = cmp::max(item_level, Some(AccessLevel::ReachableFromImplTrait));
+                self.reach(item.hir_id, item_level).generics().predicates().ty();
             }
             // Visit everything.
             hir::ItemKind::Const(..)
