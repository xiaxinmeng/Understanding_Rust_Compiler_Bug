
$ cargo build
   Compiling bar v0.1.0 (/home/andrew/src/github/bar)
note: trace_macro
 --> src/main.rs:5:9
  |
5 |         println!("{}", 1);
  |         ^^^^^^^^^^^^^^^^^^
  |
  = note: expanding `println! { "{}" , 1 }`
  = note: to `{ $crate :: io :: _print ( format_args_nl ! ( "{}" , 1 ) ) ; }`

error[E0XYZW]: `trace_macros` must be removed to finish compiling
 --> src/main.rs:2:1
  |
2 | trace_macros!(true);
  | ^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
