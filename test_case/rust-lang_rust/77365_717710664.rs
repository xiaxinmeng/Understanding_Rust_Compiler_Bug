
error[E0631]: type mismatch in closure arguments
 --> src/main.rs:4:14
  |
3 |     let f = |_| true;
  |             -------- found signature of `fn(_) -> _`
4 |     v.retain(f);
  |              ^ expected signature of `for<'r> fn(&'r i32) -> _`

error[E0271]: type mismatch resolving `for<'r> <[closure@src/main.rs:3:13: 3:21] as std::ops::FnOnce<(&'r i32,)>>::Output == bool`
 --> src/main.rs:4:7
  |
4 |     v.retain(f);
  |       ^^^^^^ expected bound lifetime parameter, found concrete lifetime

error: aborting due to 2 previous errors
