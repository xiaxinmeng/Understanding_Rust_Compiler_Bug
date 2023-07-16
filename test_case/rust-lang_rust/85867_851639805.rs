
~/forks/rust/library @ master $ rg 'impl.*SpecFromIter<.*for' -A3
alloc/src/vec/source_iter_marker.rs
24:impl<T, I> SpecFromIter<T, I> for Vec<T>
25-where
26-    I: Iterator<Item = T> + SourceIterMarker,
27-{

alloc/src/vec/spec_from_iter.rs
31:impl<T, I> SpecFromIter<T, I> for Vec<T>
32-where
33-    I: Iterator<Item = T>,
34-{
--
40:impl<T> SpecFromIter<T, IntoIter<T>> for Vec<T> {
41-    fn from_iter(iterator: IntoIter<T>) -> Self {
42-        // A common case is passing a vector into a function which immediately
43-        // re-collects into a vector. We can short circuit this if the IntoIter
--
69:impl<'a, T: 'a, I> SpecFromIter<&'a T, I> for Vec<T>
70-where
71-    I: Iterator<Item = &'a T>,
72-    T: Clone,
--
83:impl<'a, T: 'a + Clone> SpecFromIter<&'a T, slice::Iter<'a, T>> for Vec<T> {
84-    #[cfg(not(test))]
85-    fn from_iter(iterator: slice::Iter<'a, T>) -> Self {
86-        iterator.as_slice().to_vec()
