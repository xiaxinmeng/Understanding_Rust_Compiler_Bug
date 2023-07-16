
% cargo --version
cargo 1.37.0 (9edd08916 2019-08-02)
% cargo build
   Compiling bar v0.1.0 (/home/andrew/src/github/bar)
note: trace_macro
 --> src/main.rs:5:9
  |
5 |         println!("{}", 1);
  |         ^^^^^^^^^^^^^^^^^^
  |
  = note: expanding `println! { "{}" , 1 }`
  = note: to `{ $crate :: io :: _print ( format_args_nl ! ( "{}" , 1 ) ) ; }`

error[E0554]: #![feature] may not be used on the stable release channel
 --> src/main.rs:1:1
  |
1 | #![feature(trace_macros)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0554`.
error: Could not compile `bar`.

To learn more, run the command again with --verbose.
