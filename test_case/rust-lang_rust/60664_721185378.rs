bash
$ rustdoc --version
rustdoc 1.49.0-nightly (ffa2e7ae8 2020-10-24)
$ rustdoc foo.rs 
error: missing code example in this documentation
 --> foo.rs:1:1
  |
1 | / #![deny(missing_doc_code_examples)]
2 | |
3 | | /// this function has no code examples
4 | | pub fn some_function() {}
  | |_________________________^
  |
note: the lint level is defined here
 --> foo.rs:1:9
  |
1 | #![deny(missing_doc_code_examples)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^

error: missing code example in this documentation
 --> foo.rs:3:1
  |
3 | /// this function has no code examples
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
