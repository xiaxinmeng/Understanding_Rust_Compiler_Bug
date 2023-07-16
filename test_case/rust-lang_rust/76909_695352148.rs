diff
     #[inline]
     fn last(mut self) -> Option<I::Item> {
-         if self.n > 0 {
-             // nth(n) skips n+1
-             self.iter.nth(self.n - 1)?;
-         }
+        self.iter.advance_by(self.n).ok()?;
         self.iter.last()
     }
