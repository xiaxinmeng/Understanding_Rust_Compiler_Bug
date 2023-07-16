text
$ rustdoc --version
rustdoc 1.48.0-nightly (0e2c1281e 2020-09-07)
$ rustdoc foo.rs 
error: missing code example in this documentation
 --> foo.rs:1:1
  |
1 | / #![deny(missing_doc_code_examples)]
2 | | pub use std::task::RawWakerVTable;
  | |__________________________________^
  |
note: the lint level is defined here
 --> foo.rs:1:9
  |
1 | #![deny(missing_doc_code_examples)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

imperio@imperioland:~/rust/rust$ rustdoc foo.rs 
error: missing code example in this documentation
 --> foo.rs:1:1
  |
1 | / #![deny(missing_doc_code_examples)]
2 | | pub use std::task::RawWakerVTable;
  | |__________________________________^
  |
note: the lint level is defined here
 --> foo.rs:1:9
  |
1 | #![deny(missing_doc_code_examples)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
