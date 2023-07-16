diff
diff --git a/src/librustc_mir/borrow_check/borrow_set.rs b/src/librustc_mir/borrow_check/borrow_set.rs
index fd7dc7fc4b..c6e14c9353 100644
--- a/src/librustc_mir/borrow_check/borrow_set.rs
+++ b/src/librustc_mir/borrow_check/borrow_set.rs
@@ -238,16 +238,14 @@ impl<'a, 'gcx, 'tcx> Visitor<'tcx> for GatherBorrows<'a, 'gcx, 'tcx> {
         self.super_assign(block, assigned_place, rvalue, location)
     }
 
-    fn visit_place(
+    fn visit_local(
         &mut self,
-        place: &mir::Place<'tcx>,
+        &local: &mir::Local,
         context: PlaceContext<'tcx>,
         location: Location,
     ) {
-        self.super_place(place, context, location);
-
         // We found a use of some temporary TEMP...
-        if let Place::Local(temp) = place {
+        if let Place::Local(temp) = &Place::Local(local) {
             // ... check whether we (earlier) saw a 2-phase borrow like
             //
             //     TMP = &mut place
