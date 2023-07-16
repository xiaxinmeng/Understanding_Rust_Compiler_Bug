
   Compiling playground v0.0.1 (/playground)
error[E0515]: cannot return value referencing function parameter `s`
 --> src/main.rs:4:9
  |
4 |         TriggerICE(&s)
  |         ^^^^^^^^^^^--^
  |         |          |
  |         |          `s` is borrowed here
  |         returns a value referencing data owned by the current function

error: aborting due to previous error

For more information about this error, try `rustc --explain E0515`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
