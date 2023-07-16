plain
   Compiling getrandom v0.1.14
    Checking ppv-lite86 v0.2.8
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking test v0.0.0 (/checkout/library/test)
error[E0063]: missing fields `shuffle` and `shuffle_seed` in initializer of `cli::TestOpts`
   |
35 |         TestOpts {
35 |         TestOpts {
   |         ^^^^^^^^ missing `shuffle` and `shuffle_seed`
    Checking rand_core v0.5.1
For more information about this error, try `rustc --explain E0063`.
error: could not compile `test` due to previous error
warning: build failed, waiting for other jobs to finish...
