diff
-impl MyFn for fn(usize) -> usize {
+impl<F> MyFn for F where F: Fn(usize) -> usize {
