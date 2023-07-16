
error: non-ascii idents are not fully supported. (see issue #28979)
 --> test.rs:2:18
  |
2 |     let _ = (42, a̐é);
  |                  ^^
  |
  = help: add #![feature(non_ascii_idents)] to the crate attributes to enable
