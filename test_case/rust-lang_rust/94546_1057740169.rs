plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0658]: `try` expression is experimental
  --> library/std/src/sys/unix/kernel_copy/tests.rs:17:30
   |
17 |       let result: Result<()> = try {
   |  ______________________________^
18 | |         let mut source = crate::fs::OpenOptions::new()
19 | |             .read(true)
20 | |             .write(true)
...  |
59 | |         assert_eq!(&copied, b"000000wxyziklmnbcdef");
   | |_____^
   |
   = note: see issue #31436 <https://github.com/rust-lang/rust/issues/31436> for more information
   = help: add `#![feature(try_blocks)]` to the crate attributes to enable
