diff
src/librustc_mir/borrow_check/mod.rs
--- INDEX:/src/librustc_mir/borrow_check/mod.rs
+++ WORKDIR:/src/librustc_mir/borrow_check/mod.rs
@@ -1711,7 +1711,10 @@ impl<'cx, 'gcx, 'tcx> MirBorrowckCtxt<'cx, 'gcx, 'tcx> {
             Reservation(WriteKind::MutableBorrow(BorrowKind::Unique))
             | Write(WriteKind::MutableBorrow(BorrowKind::Unique)) => {
                 match self.is_mutable(place, LocalMutationIsAllowed::Yes) {
-                    Ok(root_place) => self.add_used_mut(root_place, flow_state),
+                    Ok(root_place) => {
+                        self.add_used_mut(root_place, flow_state);
+                        panic!("this is what we're looking for.");
+                    }
                     Err(_place_err) => {
                         span_bug!(span, "&unique borrow for {:?} should not fail", place);
                     }
