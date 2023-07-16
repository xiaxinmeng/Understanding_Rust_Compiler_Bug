rust
error[E0277]: the trait bound `&mut T: Clone` is not satisfied
  --> src/lib.rs:50:5
   |
48 | #[derive(Clone)]
   |          ----- in this derive macro expansion
49 | struct Foo<'a, T> {
50 |     bar: &'a mut T
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&mut T`
   |
   = help: the following other types implement trait `Clone`:
             &T
             *const T
             *mut T
   = note: `Clone` is implemented for `&T`, but not for `&mut T`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `&'a mut T: Clone` is not satisfied
  --> src/lib.rs:50:5
   |
48 | #[derive(Clone)]
   |          ----- in this derive macro expansion
49 | struct Foo<'a, T> {
50 |     bar: &'a mut T
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&'a mut T`
   |
   = help: the following other types implement trait `Clone`:
             &T
             *const T
             *mut T
   = note: `Clone` is implemented for `&'a T`, but not for `&'a mut T`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
