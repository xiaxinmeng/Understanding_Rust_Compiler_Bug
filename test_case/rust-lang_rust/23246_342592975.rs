rust
struct Quicksort<T>;  // <---- unused parameter warning
trait Sorter<T> {
  fn sort(&self,ar<T>){...}
  fn join(...);
  fn split(...);
}
impl<T> Quicksort<T> {
  pub fn sort(&self,ar<T>){
    Sorter::sort(self, ar<T>) ;
  }
}
impl<T> Sorter<T> for Quicksort<T>{
 fn join(...) {...}
 fn split(...) {...}
}
