
error[E0061]: this function takes 3 arguments but 1 argument was supplied
   --> src/tools/rls/rls-rustc/src/clippy.rs:62:9
    |
62  |         clippy_lints::register_pre_expansion_lints(&mut lint_store);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ --------------- supplied 1 argument
    |         |
    |         expected 3 arguments
