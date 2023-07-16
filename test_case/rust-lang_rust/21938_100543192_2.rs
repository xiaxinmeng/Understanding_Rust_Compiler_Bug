 console
$ rustc ./test.rs
./test.rs:1:5: 1:20 error: use of unstable library feature 'collections': recently added as part of collections reform 2
./test.rs:1 use std::vec::Drain;
                ^~~~~~~~~~~~~~~
./test.rs:1:5: 1:20 warning: unused import, #[warn(unused_imports)] on by default
./test.rs:1 use std::vec::Drain;
                ^~~~~~~~~~~~~~~
error: aborting due to previous error
