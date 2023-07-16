
error[E0585]: found a documentation comment that doesn't document anything
 --> /tmp/test.rs:3:5
  |
3 |     /// the id of the parent core
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: doc comments must come before what they document, maybe a comment was intended with `//`?

error: expected `,`, or `}`, found `/// the id of the parent core`
 --> /tmp/test.rs:3:5
  |
3 |     /// the id of the parent core
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: struct fields should be separated by commas

error[E0601]: main function not found

error: aborting due to 3 previous errors

You've got a few errors: E0585, E0601
If you want more information on an error, try using "rustc --explain E0585"
