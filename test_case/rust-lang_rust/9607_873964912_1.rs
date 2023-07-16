
error[E0277]: the trait bound `T: Clone` is not satisfied
  --> src/main.rs:15:5
   |
15 |     x: Foo<T>
   |     ^^^^^^^^^ expected an implementor of trait `Clone`
   |
   = note: required because of the requirements on the impl of `Clone` for `Foo<T>`
   = note: required by `clone`
   = note: this error originates from the derive macro `#[derive(Clone]`
   = note: in Nightly builds, run with -Z macro-backtrace for more info
