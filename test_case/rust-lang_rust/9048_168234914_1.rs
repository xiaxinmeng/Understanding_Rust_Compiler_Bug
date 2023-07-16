
hello.rs:3:37: 3:52 error: type `core::option::Option<&_>` cannot be dereferenced
hello.rs:3     let f = |n: usize| { n == 0 || (*rec_f.as_ref())(n - 1) };
                                               ^~~~~~~~~~~~~~~
hello.rs:3:13: 3:62 error: mismatched types:
 expected `&core::option::Option<[closure@hello.rs:3:13: 3:62 rec_f:_]>`,
    found `_`
(cyclic type of infinite size) [E0308]
hello.rs:3     let f = |n: usize| { n == 0 || (*rec_f.as_ref())(n - 1) };
                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:3:13: 3:62 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to 2 previous errors
