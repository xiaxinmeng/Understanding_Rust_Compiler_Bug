
> cargo build
src/lib.rs:7:5: 7:19 error: type `Test` is inaccessible
src/lib.rs:7 use foo::sub::Test;
                 ^~~~~~~~~~~~~~
src/lib.rs:7:5: 7:19 note: module `sub` is private
src/lib.rs:7 use foo::sub::Test;
                 ^~~~~~~~~~~~~~
error: aborting due to previous error
Could not compile `bar`.
