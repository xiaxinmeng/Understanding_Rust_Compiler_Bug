
error[E0631]: type mismatch in function arguments
 --> src/main.rs:8:24
  |
5 | fn foo<T: Foo>(x: T) { }
  | -------------------- found signature of `fn(_) -> _`
...
8 |     let x: &Fn(&i32) = &foo;
  |                        ^^^^ expected signature of `for<'r> fn(&'r i32) -> _`
  |
  = note: required for the cast to the object type `for<'r> std::ops::Fn(&'r i32)`

error[E0271]: type mismatch resolving `for<'r> <fn(_) {foo::<_>} as std::ops::FnOnce<(&'r i32,)>>::Output == ()`
 --> src/main.rs:8:24
  |
8 |     let x: &Fn(&i32) = &foo;
  |                        ^^^^ expected bound lifetime parameter, found concrete lifetime
  |
  = note: required for the cast to the object type `for<'r> std::ops::Fn(&'r i32)`

error: aborting due to 2 previous errors
