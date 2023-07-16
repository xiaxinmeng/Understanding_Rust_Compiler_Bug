
error[E0038]: the trait `T` cannot be made into an object
 --> src/main.rs:7:15
  |
1 | trait T: Clone {}
  |       -  ----- ...because it requires `Self: Sized`
  |       |
  |       this trait cannot be made into an object...
...
7 |   let x: &T = & S{};
  |               ^^^ the trait `T` cannot be made into an object
  |
  = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn T>` for `&S`
  = note: required by cast to type `&dyn T`

error[E0038]: the trait `T` cannot be made into an object
 --> src/main.rs:7:10
  |
1 | trait T: Clone {}
  |       -  ----- ...because it requires `Self: Sized`
  |       |
  |       this trait cannot be made into an object...
...
7 |   let x: &T = & S{};
  |          ^^ the trait `T` cannot be made into an object
