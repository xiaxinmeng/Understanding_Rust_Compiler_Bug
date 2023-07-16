
error[E0061]: this function takes 1 argument but 2 arguments were supplied
  --> src/tools/rls/rls-rustc/src/clippy.rs:62:9
   |
62 |         clippy_lints::register_pre_expansion_lints(&mut lint_store, &conf);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ---------------  ----- supplied 2 arguments
   |         |
   |         expected 1 argument

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rls-rustc`.
