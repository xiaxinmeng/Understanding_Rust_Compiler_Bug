text
error[E0015]: cannot call non-const operator in constants
  --> /home/ubuntu/projects/rust/src/test/ui/transmutability/arrays/should_have_correct_length.rs:14:52
   |
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, { Assume::SAFETY + Assume::VALIDITY }>
   |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: impl defined here, but it is not `const`
  --> /home/ubuntu/projects/rust/library/core/src/mem/transmutability.rs:65:1
   |
LL | impl const core::ops::Add for Assume {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: calls in constants are limited to constant functions, tuple structs and tuple variants
