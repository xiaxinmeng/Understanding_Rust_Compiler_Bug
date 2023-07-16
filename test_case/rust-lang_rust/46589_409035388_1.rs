rust
   Compiling playground v0.0.1 (file:///playground)
error[E0499]: cannot borrow `self.buf_read` as mutable more than once at a time
  --> src/main.rs:12:23
   |
12 |             let buf = self.buf_read.fill_buf();
   |                       ^^^^^^^^^^^^^ mutable borrow starts here in previous iteration of loop
   |
note: borrowed value must be valid for the lifetime 'a as defined on the method body at 10:17...
  --> src/main.rs:10:17
   |
10 |     pub fn next<'a>(&'a mut self) -> &'a str {
   |                 ^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.

