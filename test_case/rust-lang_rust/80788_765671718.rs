plain
    Checking clippy_lints v0.1.51 (/checkout/src/tools/clippy/clippy_lints)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> src/tools/clippy/clippy_lints/src/utils/usage.rs:30:10
   |
30 |         .walk_expr(expr);
   |          |
   |          expected 2 arguments

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/tools/clippy/clippy_lints/src/loops.rs:2067:10
     |
2067 |         .walk_expr(body);
     |          |
     |          expected 2 arguments

error: aborting due to 2 previous errors
