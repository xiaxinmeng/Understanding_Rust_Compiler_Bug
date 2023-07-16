
$ rustc foo.rs   
foo.rs:1:1: 1:18 warning: found staticlib `bar` instead of rlib `bar`, please compile using --crate-type rlib instead.
foo.rs:1 extern crate bar;
         ^~~~~~~~~~~~~~~~~
foo.rs:1:1: 1:18 error: can't find crate for `bar`
foo.rs:1 extern crate bar;
         ^~~~~~~~~~~~~~~~~
error: aborting due to previous error
