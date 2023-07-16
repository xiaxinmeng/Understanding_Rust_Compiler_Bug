console
$ cat test.md
foo
$ cat test.rs
#![doc(include = "test.md")]
#![feature(external_doc)]
#![deny(missing_docs)]

fn main() {}
$ rustc +nightly test.rs
error: missing documentation for crate
 --> test.rs:1:1
  |
1 | / #![doc(include = "test.md")]
2 | | #![feature(external_doc)]
3 | | #![deny(missing_docs)]
4 | |
5 | | fn main() {}
  | |____________^
  |
note: lint level defined here
 --> test.rs:3:9
  |
3 | #![deny(missing_docs)]
  |         ^^^^^^^^^^^^

error: aborting due to previous error
