
#![feature(iterator_fold_self)]
fn arr_max<T: Ord, const N: usize>(v: [T; N]) -> T {
  assert(N > 0); // This should get compiled out when N > 0 and when N == 0 this will just be a panic.
  // I would think the unwrap would be compiled out but unsure.
  Array::into_iter(v).fold_first(|a, b| a.max(b)).unwrap()
}
