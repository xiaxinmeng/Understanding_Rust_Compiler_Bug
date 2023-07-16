plain
    Checking ppv-lite86 v0.2.8
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking test v0.0.0 (/checkout/library/test)
    Checking rand_core v0.5.1
error[E0063]: missing fields `shuffle` and `shuffle_seed` in initializer of `cli::TestOpts`
   |
35 |         TestOpts {
35 |         TestOpts {
   |         ^^^^^^^^ missing `shuffle` and `shuffle_seed`
    Checking rand_chacha v0.2.2
    Checking rand_xorshift v0.2.0
For more information about this error, try `rustc --explain E0063`.
error: could not compile `test` due to previous error
