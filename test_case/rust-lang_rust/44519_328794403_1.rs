
error: Expected 2 arguments, got something else
  --> src/main.rs:5:26
   |
5  |     ($($args:tt),*) => { compile_error!("Expected 2 arguments, got something else") };
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
18 |     println!("{:?}", cons!());
   |                      ------- in this macro invocation

error: Expected 2 arguments, got something else
  --> src/main.rs:5:26
   |
5  |     ($($args:tt),*) => { compile_error!("Expected 2 arguments, got something else") };
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
19 |     println!("{:?}", cons!(1, 2, 3, 4));
   |                      ----------------- in this macro invocation

error: aborting due to 2 previous errors
