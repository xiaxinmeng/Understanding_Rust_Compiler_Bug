plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error: missing `in` in `for` loop
    |
    |
216 |                         for &borrow_index inborrow_indices {
    |                                          ^ help: try adding `in` here

error[E0425]: cannot find value `inborrow_indices` in this scope
    |
    |
216 |                         for &borrow_index inborrow_indices {
    |                                           ^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `borrow_indices`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_borrowck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
