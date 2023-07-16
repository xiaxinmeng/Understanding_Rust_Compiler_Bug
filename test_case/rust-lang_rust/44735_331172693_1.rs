
error[E0631]: type mismatch in closure arguments
 --> test.rs:5:35
  |
5 | fn bar<'a>(f: fn(*mut &'a u32)) { foo(f); }
  |                                   ^^^
  |                                   |
  |                                   expected argument of type `*mut &u32`
  |                                   takes argument of type `*mut &'a u32`
  |
  = note: required by `foo`

error[E0271]: type mismatch resolving `for<'r> <fn(*mut &'a u32) as std::ops::FnOnce<(*mut &'r u32,)>>::Output == ()`
 --> test.rs:5:35
  |
5 | fn bar<'a>(f: fn(*mut &'a u32)) { foo(f); }
  |                                   ^^^ expected bound lifetime parameter, found concrete lifetime
  |
  = note: required by `foo`

error: aborting due to 2 previous errors
