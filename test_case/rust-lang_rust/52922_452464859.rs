
error: any use of this value will cause an error
 --> src/main.rs:8:1
  |
8 | const BAD: usize = unsafe { Foo { a: &1 }.b * 2};
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^-------------------^^
  |                             |
  |                             "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
  |
  = note: #[deny(const_err)] on by default
