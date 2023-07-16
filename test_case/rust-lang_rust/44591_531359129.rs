rust
trait Sized {}
trait FromIterator<A> {}
trait Iterator {
  type Item;
}
struct Vec<T> {}
struct Tup1<A> {}
struct Tup2<A, B> {}
// impl for FromIterator here; you may need to define IntoIterator
// continue with the rest of your program
