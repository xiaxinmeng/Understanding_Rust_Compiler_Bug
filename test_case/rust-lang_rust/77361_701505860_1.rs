
error: `from_generator` is not yet stable as a const fn
 --> src/lib.rs:1:53
  |
1 | fn regress() -> [(); { core::mem::ManuallyDrop::new(async { 0 }); 4 }] {
  |                                                     ^^^^^^^^^^^
  |
  = help: add `#![feature(gen_future)]` to the crate attributes to enable
