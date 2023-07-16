
$ cargo run
   Compiling expr-proc-macro-def-site v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macro-def-site)
error[E0428]: the name `do_something` is defined multiple times
 --> src/main.rs:6:5
  |
6 |     outer!();
  |     ^^^^^^^^
  |     |
  |     `do_something` redefined here
  |     previous definition of the value `do_something` here
  |
  = note: `do_something` must be defined only once in the value namespace of this block
  = note: this error originates in the macro `inner_using_outer_declarations_via_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0618]: expected function, found `i32`
 --> src/main.rs:6:5
  |
6 |     outer!();
  |     ^^^^^^^^
  |     |
  |     call expression requires function
  |     `do_something` has type `i32`
  |
  = note: this error originates in the macro `inner_using_outer_declarations_via_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0428, E0618.
For more information about an error, try `rustc --explain E0428`.
error: could not compile `expr-proc-macro-def-site` due to 2 previous errors
