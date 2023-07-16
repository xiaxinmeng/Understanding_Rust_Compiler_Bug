
foo.rs:11:1: 11:18 error: found crate `foo` compiled by an incompatible version of rustc [E0514]
foo.rs:11 extern crate foo;
          ^~~~~~~~~~~~~~~~~
foo.rs:11:1: 11:18 help: please recompile that crate using this compiler (rustc 1.10.0-dev (e324784f7 2016-07-02))
foo.rs:11:1: 11:18 note: crate `foo` path #1: .. compiled by "an unknown compiler"
error: aborting due to previous error
