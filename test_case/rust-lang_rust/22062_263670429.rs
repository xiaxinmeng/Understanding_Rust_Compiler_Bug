
error[E0412]: associated type `DerefMut::Target` is undefined or not in scope
 --> src\lib.rs:6:16
  |
6 |   member: *mut <T as DerefMut>::Target,
  |                ^^^^^^^^^^^^^^^^^^^^^^^ undefined or not in scope
  |
  = help: no candidates by the name of `Target` found in your project; maybe you misspelled the name or forgot to import an external crate?

error[E0412]: associated type `DerefMut::Target` is undefined or not in scope
  --> src\lib.rs:13:39
   |
13 |   fn member<'a>(&'a self) -> &'a *mut <T as DerefMut>::Target {
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^ undefined or not in scope
   |
   = help: no candidates by the name of `Target` found in your project; maybe you misspelled the name or forgot to import an external crate?

error[E0392]: parameter `T` is never used
 --> src\lib.rs:3:12
  |
3 | struct Foo<T>
  |            ^ unused type parameter
  |
  = help: consider removing `T` or using a marker such as `std::marker::PhantomData`

error: aborting due to previous error
