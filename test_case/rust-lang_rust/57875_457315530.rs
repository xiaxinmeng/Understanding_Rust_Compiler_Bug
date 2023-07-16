
error[E0631]: type mismatch in closure arguments
 --> src/main.rs:8:15
  |
7 |     let lambda = |&x, &y| x + y;
  |                  -------------- found signature of `fn(&_, &_) -> _`
8 |     let foo = Foo {
  |               ^^^ expected signature of `for<'r, 's> fn(&'r i32, &'s i32) -> _`
  |
note: required by `Foo`
 --> src/main.rs:1:1
  |
1 | struct Foo<T, F: Fn(&T, &T) -> T> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/main.rs:7:18: 7:32] as std::ops::FnOnce<(&'r i32, &'s i32)>>::Output == i32`
 --> src/main.rs:8:15
  |
8 |     let foo = Foo {
  |               ^^^ expected bound lifetime parameter, found concrete lifetime
  |
note: required by `Foo`
 --> src/main.rs:1:1
  |
1 | struct Foo<T, F: Fn(&T, &T) -> T> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
