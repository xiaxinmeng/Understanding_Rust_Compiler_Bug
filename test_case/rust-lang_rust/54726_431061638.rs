
error[E0658]: non-builtin inner attributes are unstable (see issue #54726)
 --> src/lib.rs:3:1
  |
3 | #![foo]
  | ^^^^^^^
  |
  = help: add #![feature(custom_inner_attributes)] to the crate attributes to enable

error[E0658]: The attribute `foo` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
 --> src/lib.rs:3:4
  |
3 | #![foo]
  |    ^^^
  |
  = help: add #![feature(custom_attribute)] to the crate attributes to enable
