rust
./dev/code/rust$ rustc +stage1 ./dev/tests/data/rust_52895_confusing_mismatch.rs 
error[E0512]: transmute called with types of different sizes
  --> ./dev/tests/data/rust_52895_confusing_mismatch.rs:10:27
   |
10 |     let _: Foo = unsafe { std::mem::transmute(x) };
   |                           ^^^^^^^^^^^^^^^^^^^
   |
   = note: source type: Foo (this type's size can vary)
   = note: target type: Foo (this type's size can vary)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0512`.
