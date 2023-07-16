
$ rustc foo.rs
foo.rs:5:5: 5:13 error: illegal left-hand side expression [E0070]
foo.rs:5     A::C = 1;
             ^~~~~~~~
foo.rs:5:12: 5:13 error: mismatched types: expected `A::C`, found `_` (expected struct A::C, found integral variable)
foo.rs:5     A::C = 1;
                    ^
error: aborting due to 2 previous errors
