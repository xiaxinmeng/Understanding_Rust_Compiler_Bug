
$ rustc lib10814.rs --target=i686-unknown-linux-gnu
$ rustc 10814.rs -L .
10814.rs:1:1: 1:23 error: found incorrect triple for crate `lib10814`
10814.rs:1 extern crate lib10814;
           ^~~~~~~~~~~~~~~~~~~~~~
10814.rs:1:1: 1:23 note: expected triple of x86_64-unknown-linux-gnu
10814.rs:1 extern crate lib10814;
           ^~~~~~~~~~~~~~~~~~~~~~
10814.rs:1:23: 1:23 note: crate `lib10814` path #1, triple i686-unknown-linux-gnu: .../triage/liblib10814.rlib
error: aborting due to previous error
