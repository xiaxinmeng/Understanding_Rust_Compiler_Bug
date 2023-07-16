
error[E0499]: cannot borrow `*vec` as mutable more than once at a time
 --> src/main.rs:6:9
  |
1 | fn f(vec: &mut Vec<u8>) -> &u8 {
  |           - let's call the lifetime of this reference `'1`
2 |     if let Some(n) = vec.iter_mut().find(|n| **n == 1) {
  |                      --- first mutable borrow occurs here
3 |         *n = 10;
4 |         n
  |         - returning this value requires that `*vec` is borrowed for `'1`
5 |     } else {
6 |         vec.push(10);
  |         ^^^ second mutable borrow occurs here

error[E0502]: cannot borrow `*vec` as immutable because it is also borrowed as mutable
 --> src/main.rs:7:9
  |
1 | fn f(vec: &mut Vec<u8>) -> &u8 {
  |           - let's call the lifetime of this reference `'1`
2 |     if let Some(n) = vec.iter_mut().find(|n| **n == 1) {
  |                      --- mutable borrow occurs here
3 |         *n = 10;
4 |         n
  |         - returning this value requires that `*vec` is borrowed for `'1`
...
7 |         vec.last().unwrap()
  |         ^^^ immutable borrow occurs here
