
warning: this constant cannot be used
 --> src/main.rs:8:1
  |
8 | const BAD: usize = unsafe { Foo { a: &1 }.b * 2};
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------^^
  |                             |
  |                             "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
  |
note: lint level defined here
 --> src/main.rs:1:9
  |
1 | #![warn(const_err)]
  |         ^^^^^^^^^

error[E0080]: referenced constant has errors
  --> src/main.rs:10:13
   |
8  | const BAD: usize = unsafe { Foo { a: &1 }.b * 2};
   |                             ------------------- "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
9  | fn main() {
10 |     let f = BAD;
   |             ^^^

error[E0080]: could not evaluate constant
  --> src/main.rs:10:13
   |
10 |     let f = BAD;
   |             ^^^ referenced constant has errors

error[E0080]: constant evaluation error
 --> src/main.rs:8:1
  |
8 | const BAD: usize = unsafe { Foo { a: &1 }.b * 2};
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------^^
  |                             |
  |                             "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
