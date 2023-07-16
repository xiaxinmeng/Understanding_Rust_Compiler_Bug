diff
- impl<I: Iterator> IntoIterator for I {
+ impl<I: ~const Iterator> const IntoIterator for I {
