 diff
diff --git a/src/liballoc/rc.rs b/src/liballoc/rc.rs
index 9109e9d..8318607 100644
--- a/src/liballoc/rc.rs
+++ b/src/liballoc/rc.rs
@@ -784,7 +784,7 @@ impl<T: ?Sized> Drop for Weak<T> {
             let ptr = *self._ptr;
             let thin = ptr as *const ();

-            if !thin.is_null() && thin as usize != mem::POST_DROP_USIZE {
+            if thin.is_null() || thin as usize == mem::POST_DROP_USIZE {
                 self.dec_weak();
                 // the weak count starts at 1, and will only go to zero if all
                 // the strong pointers have disappeared.
