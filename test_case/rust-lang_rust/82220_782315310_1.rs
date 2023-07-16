
   Compiling playground v0.0.1 (/playground)
error[E0618]: expected function, found `usize`
 --> src/main.rs:4:5
  |
1 | const FOO: usize = 0;
  | --------------------- `usize` defined here
...
4 |     FOO();
  |     ^^^--
  |     |
  |     call expression requires function

error: aborting due to previous error

For more information about this error, try `rustc --explain E0618`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
