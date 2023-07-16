
error[E0271]: type mismatch resolving `for<'r> <&[closure@file4.rs:5:10: 5:17] as std::ops::FnOnce<(&'r u8,)>>::Output == ()`
 --> file4.rs:5:5
  |
1 | fn foo<F>(_compare: F) where F: Fn(&u8) {
  |    ---                          ------- required by this bound in `foo`
...
5 |     foo(&|_a| {});
  |     ^^^ expected bound lifetime parameter, found concrete lifetime
