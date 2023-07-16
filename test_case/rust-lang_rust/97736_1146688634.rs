rust
impl<T> IntoIterator for IterRef<T>
where
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::IntoIter: Iterator,
{
  // What do we put as type Iter, and as type Item? 
  //We can't put `type Item = <<&'a T as IntoIterator>::IntoIter as Iterator>::Item`, since we have no `'a` in scope.
}
