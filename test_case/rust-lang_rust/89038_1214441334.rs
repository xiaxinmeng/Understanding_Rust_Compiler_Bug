
error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): the trait bound `Bar: Eq` is not satisfied
  --> src/main.rs:13:15
   |
13 |      x.equals(&x); 
   |        ------ ^^ the trait `Eq` is not implemented for `Bar`
   |        |
   |        required by a bound introduced by this call
   |
note: required by a bound in `Foo::<T>::equals`
  --> src/main.rs:6:53
   |
6  |      fn equals(&self, u: &Foo<T>) -> bool where T : Eq { 
   |                                                     ^^ required by this bound in `Foo::<T>::equals`
help: consider annotating `Bar` with `#[derive(Eq)]`
   |
1  | #[derive(Eq)]
   |
