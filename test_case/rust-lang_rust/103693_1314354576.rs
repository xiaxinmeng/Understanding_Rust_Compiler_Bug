plain
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error[E0599]: no method named `is_sized` found for reference `&F` in the current scope
   --> compiler/rustc_abi/src/layout.rs:885:27
    |
885 |             assert!(field.is_sized());
    |
help: one of the expressions' fields has a method of the same name
    |
    |
885 |             assert!(field.abi.is_sized());
help: there is a method with a similar name
    |
    |
885 |             assert!(field.is_unsized());

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_abi` due to previous error
warning: build failed, waiting for other jobs to finish...
