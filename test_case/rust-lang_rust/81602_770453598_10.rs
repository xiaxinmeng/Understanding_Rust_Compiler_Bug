rust,ignore
-    /// for term in self.iter_mut() {
-    ///   if term.is_boxed() {
-    ///       ...
-    ///   }
-    /// }
     fn iter_mut<'a>(&mut self) -> IterMut<'a, Self>;
 }
 

