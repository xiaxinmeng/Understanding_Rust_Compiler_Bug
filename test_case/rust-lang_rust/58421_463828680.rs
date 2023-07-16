diff
  impl<T> BinaryHeap<T>
- where
-     T: Ord,
  {
      fn iter(&self) -> Iter<'_, T>;
      fn peek(&self) -> Option<&T>;
      fn capacity(&self) -> usize;
      fn reserve_exact(&mut self, additional: usize);
      fn reserve(&mut self, additional: usize);
      fn shrink_to_fit(&mut self);
      fn shrink_to(&mut self, min_capacity: usize);
      fn into_vec(self) -> Vec<T>;
      fn len(&self) -> usize;
      fn is_empty(&self) -> bool;
      fn drain(&mut self) -> Drain<'_, T>;
      fn clear(&mut self);
  }

  impl<T> Debug for BinaryHeap<T>
  where
      T: Debug,
-     T: Ord;

  impl<T> IntoIterator for BinaryHeap<T>
- where
-     T: Ord;

  impl<'a, T> IntoIterator for &'a BinaryHeap<T>
- where
-     T: Ord;
