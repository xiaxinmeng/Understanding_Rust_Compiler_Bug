plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `macro_parent_module_id` in this scope
   --> src/librustdoc/visit_ast.rs:111:36
    |
111 |             assert_eq!(cur_mod.id, macro_parent_module_id);
    |                                    ^^^^^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `macro_parent_module`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustdoc`
