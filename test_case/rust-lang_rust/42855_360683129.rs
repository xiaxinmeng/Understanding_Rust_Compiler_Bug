
error[E0593]: function is expected to take 2 arguments, but it takes 1 argument
 --> src/main.rs:2:5
  |
2 |     bar(1, foo);
  |     ^^^ expected function that takes 2 arguments
...
5 | fn foo(a: u16) {}
  | -------------- takes 1 argument
  |
note: required by `bar`
 --> src/main.rs:7:1
  |
7 | fn bar<F>(a: u16, mut f: F) where F: FnMut(u16, u8) {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
