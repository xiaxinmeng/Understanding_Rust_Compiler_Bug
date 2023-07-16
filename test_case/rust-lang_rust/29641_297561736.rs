rust
error: box pattern syntax is experimental (see issue #29641)
 --> a.rs:2:9
  |
2 |     let box a = Box::new(1);
  |         ^^^^^
  |
  = help: add #![feature(box_patterns)] to the crate attributes to enable

error: aborting due to previous error
