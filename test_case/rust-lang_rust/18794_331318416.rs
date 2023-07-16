
error[E0281]: type mismatch: `fn(_) {foo::<_>}` implements the trait `std::ops::Fn<(_,)>`, but the trait `for<'r> std::ops::Fn<(&'r i32,)>` is required
 --> src/main.rs:6:24
  |
6 |     let x: &Fn(&i32) = &foo;
  |                        ^^^^ expected concrete lifetime, found bound lifetime parameter
  |
  = note: required for the cast to the object type `for<'r> std::ops::Fn(&'r i32)`

error[E0271]: type mismatch resolving `for<'r> <fn(_) {foo::<_>} as std::ops::FnOnce<(&'r i32,)>>::Output == ()`
 --> src/main.rs:6:24
  |
6 |     let x: &Fn(&i32) = &foo;
  |                        ^^^^ expected bound lifetime parameter, found concrete lifetime
  |
  = note: required for the cast to the object type `for<'r> std::ops::Fn(&'r i32)`
