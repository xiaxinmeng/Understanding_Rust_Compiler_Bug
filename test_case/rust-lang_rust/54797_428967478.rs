rust
warning[E0507]: cannot move out of static item
  --> src/lib.rs:11:16
   |
11 |             f(&TL_COMMAND_BUFFER_WRITER.unwrap());
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of static item
   |
   = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
           It represents potential unsoundness in your code.
           This warning will become a hard error in the future.
