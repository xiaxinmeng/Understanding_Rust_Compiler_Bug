rust
./dev/code/rust$ rustc +stage1 ./dev/tests/data/rust_52895_ice.rs 
error[E0512]: transmute called with types of different sizes
  -->  ./dev/tests/data/rust_52895_ice.rs:11:27
   |
11 |     let _: Foo = unsafe { std::mem::transmute(0u8) };
   |                           ^^^^^^^^^^^^^^^^^^^
   |
   = note: source type: u8 (8 bits)
   = note: target type: Foo (this type's size can vary)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0512`.
