
../src/librustc_typeck/check/method/suggest.rs:18:44: 18:55 error: unresolved import `middle::ty::AsPredicate`. There is no `AsPredicate` in `rustc::middle::ty`
../src/librustc_typeck/check/method/suggest.rs:18 use middle::ty::{self, Ty, ToPolyTraitRef, AsPredicate};
                                                                                             ^~~~~~~~~~~
error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_typeck] Error 101
