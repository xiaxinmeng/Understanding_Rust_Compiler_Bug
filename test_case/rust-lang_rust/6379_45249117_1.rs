
$ rustc foo.rs
foo.rs:1:5: 1:8 error: unresolved import. maybe a missing `extern crate bar`?
foo.rs:1 use bar::baz;
             ^~~
foo.rs:1:5: 1:13 error: failed to resolve import `bar::baz`
foo.rs:1 use bar::baz;
             ^~~~~~~~
error: aborting due to 2 previous errors
