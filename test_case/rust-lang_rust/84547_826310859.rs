plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `rustc_mir::const_eval::is_min_const_fn`
  --> src/librustdoc/clean/inline.rs:13:5
13 | use rustc_mir::const_eval::is_min_const_fn;
   |     ^^^^^^^^^^^^^^^^^^^^^^^---------------
   |     |                      |
   |     |                      help: a similar name exists in the module: `is_const_fn`
   |     |                      help: a similar name exists in the module: `is_const_fn`
   |     no `is_min_const_fn` in `const_eval`

error[E0432]: unresolved import `rustc_mir::const_eval::is_min_const_fn`
  --> src/librustdoc/clean/mod.rs:25:42
   |
25 | use rustc_mir::const_eval::{is_const_fn, is_min_const_fn, is_unstable_const_fn};
   |                                          |
   |                                          |
   |                                          no `is_min_const_fn` in `const_eval`
   |                                          help: a similar name exists in the module: `is_const_fn`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc`
