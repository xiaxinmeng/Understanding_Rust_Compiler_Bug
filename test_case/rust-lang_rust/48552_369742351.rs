
warning: a method with this name will be added to the standard library in the future
  --> $DIR/inference_unstable.rs:26:20
   |
LL |     assert_eq!('x'.ipu_flatten(), 1);
   |                    ^^^^^^^^^^^
   |
   = note: #[warn(unstable_name_collision)] on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see pr #48552 <https://github.com/rust-lang/rust/pull/48552>
   = help: call with fully qualified syntax `inference_unstable_itertools::IpuItertools::ipu_flatten(...)` to keep using the current method
   = note: add #![feature(ipu_flatten)] to the crate attributes to enable `inference_unstable_iterator::IpuIterator::ipu_flatten`
