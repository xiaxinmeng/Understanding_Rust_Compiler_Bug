plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0547]: missing 'issue'
    --> library/core/src/option.rs:1374:5
     |
1374 |     #[unstable(feature = "option_not")]

error[E0547]: missing 'issue'
    --> library/core/src/option.rs:1375:5
     |
     |
1375 |     #[rustc_const_unstable(feature = "option_not")]

For more information about this error, try `rustc --explain E0547`.
error: could not compile `core` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
