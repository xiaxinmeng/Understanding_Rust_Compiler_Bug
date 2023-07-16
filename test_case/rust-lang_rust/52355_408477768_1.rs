
$ rustc foo.rs "-Zcrate-attr=feature(10)"
error[E0556]: malformed feature, expected just one word
 --> foo.rs:1:9
  |
1 | #![allow(dead_code)]
  |         ^^

error[E0658]: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
 --> foo.rs:1:1
  |
1 | #![allow(dead_code)]
  | ^^^^^^^^^^^
  |
  = help: add #![feature(attr_literals)] to the crate attributes to enable
