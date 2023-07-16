diff
https://github.com/rust-lang/rust/issues/52262#issuecomment-537168375

diff --git a/src/librustc_mir/borrow_check/mod.rs b/src/librustc_mir/borrow_check/mod.rs
index cfa211ad5af..8f4dbb3bb81 100644
--- a/src/librustc_mir/borrow_check/mod.rs
+++ b/src/librustc_mir/borrow_check/mod.rs
@@ -1946,11 +1946,11 @@ impl<'cx, 'tcx> MirBorrowckCtxt<'cx, 'tcx> {
                 ) {
                     // rust-lang/rust#46908: In pure NLL mode this code path should
                     // be unreachable (and thus we signal an ICE in the else branch here).
-                    span_bug!(
+                    self.infcx.tcx.sess.delay_span_bug(
                         span,
-                        "Accessing `{:?}` with the kind `{:?}` shouldn't be possible",
+                        format!("Accessing `{:?}` with the kind `{:?}` shouldn't be possible",
                         place,
-                        kind,
+                        kind).as_str()
                     );
                 }
                 return false;
