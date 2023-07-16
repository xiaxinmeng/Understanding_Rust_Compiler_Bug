
[00:27:20] error[E0080]: could not evaluate static initializer
[00:27:20]    --> libsyntax/ast.rs:929:1
[00:27:20]     |
[00:27:20] 929 | static_assert!(EXPR_IS_AT_MOST_88_BYTES: std::mem::size_of::<Expr>() <= 88);
[00:27:20]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the len is 1 but the index is 1
[00:27:20]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:27:20] 
[00:27:20] error: aborting due to previous error
