plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0423]: expected function, found macro `assert_eq`
   --> compiler/rustc_typeck/src/coherence/builtin.rs:527:21
    |
527 |                     assert_eq(i, 0, "expected JustMetadata to be coerced in field 0");
    |
    |
help: use `!` to invoke the macro
    |
527 |                     assert_eq!(i, 0, "expected JustMetadata to be coerced in field 0");

For more information about this error, try `rustc --explain E0423`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
