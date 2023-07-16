
error[E0283]: type annotations required: cannot resolve `std::option::Option<&'a str>: Tr`
 --> src/main.rs:5:1
  |
5 | impl<'a> Tr for S<'a> where Option<&'a str>: Tr, Option<&'static str>: Tr { }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: required by `Tr`
 --> src/main.rs:3:1
  |
3 | trait Tr { }
  | ^^^^^^^^
