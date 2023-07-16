
--- INDEX:/src/libcore/iter.rs
+++ WORKDIR:/src/libcore/iter.rs
@@ -1127,7 +1127,11 @@
 #[unstable(feature = "core", reason = "trait is experimental")]
 impl<I> RandomAccessIterator for Rev<I> where I: DoubleEndedIterator + RandomAccessIterator {
     #[inline]
     fn indexable(&self) -> usize { self.iter.indexable() }
     #[inline]
     fn idx(&mut self, index: usize) -> Option<<I as Iterator>::Item> {
         let amt = self.indexable();
-        self.iter.idx(amt - index - 1)
+        if amt > index {
+            self.iter.idx(amt - index - 1)
+        } else {
+            None
+        }
     }
 }

