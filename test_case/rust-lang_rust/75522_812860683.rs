plain
    |
219 |     vec![
    |         - closing delimiter possibly meant for this
...
531 |             o.optmulti(
...
544 |     ]
    |     ^ mismatched closing delimiter


error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> src/librustdoc/lib.rs:531:15
    |
531 |               o.optmulti(
532 |                   "",
    |                   --
    |                   --
533 |                   "emit",
    |                   ------
534 |                   "Comma separated list of types of output for rustdoc to emit",
    |                   -------------------------------------------------------------
535 |                   "[unversioned-shared-resources,toolchain-shared-resources,invocation-specific]",
    |                   -------------------------------------------------------------------------------
536 | /         stable("no-source", |o| {
537 | |             o.optflag(
538 | |                 "",
539 | |                 "no-source",
542 | |             )
543 | |         }),
    | |__________- supplied 5 arguments
    |
    |
note: associated function defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs:261:12
    |
261 |     pub fn optmulti(

error[E0308]: mismatched types
   --> src/librustdoc/lib.rs:530:26
    |
    |
530 |           unstable("emit", |o| {
    |  __________________________^
531 | |             o.optmulti(
532 | |                 "",
533 | |                 "emit",
543 | |         }),
544 | |     ]
    | |_____^ expected fn pointer, found closure
    |
    |
    = note: expected fn pointer `for<'r> fn(&'r mut rustc_session::getopts::Options) -> &'r mut rustc_session::getopts::Options`
                  found closure `[closure@src/librustdoc/lib.rs:530:26: 544:6]`
note: closures can only be coerced to `fn` types if they do not capture any variables
    |
    |
536 |         stable("no-source", |o| {
    |         ^^^^^^ `stable` captured here
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0061, E0308.
For more information about an error, try `rustc --explain E0061`.
