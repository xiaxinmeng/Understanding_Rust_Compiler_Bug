text
error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
 --> src/lib.rs:4:9
  |
4 |         asm!("");
  |         ^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/29722
  = help: add `#![feature(asm)]` to the crate attributes to enable
