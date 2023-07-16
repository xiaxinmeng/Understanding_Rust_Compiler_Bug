
$ rustc foo.rs
foo.rs:15:8: 15:23 error: illegal left-hand side expression [E0070]
foo.rs:15   test(A::C = B::new());
                 ^~~~~~~~~~~~~~~
foo.rs:15:8: 15:23 error: mismatched types: expected `Box<A::B>`, found `()` (expected box, found ())
foo.rs:15   test(A::C = B::new());
                 ^~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
