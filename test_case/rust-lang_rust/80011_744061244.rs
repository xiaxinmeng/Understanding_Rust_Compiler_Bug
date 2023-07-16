diff
  impl<I: Iterator> Peekable<I> {
+     pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item>;

+     pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
+     where
+         T: ?Sized,
+         I::Item: PartialEq<T>;
  }
