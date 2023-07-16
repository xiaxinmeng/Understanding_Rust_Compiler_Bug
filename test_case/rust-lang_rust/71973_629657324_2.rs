
error: constant expression depends on a generic parameter
  --> /home/programming/rust/src/test/ui/__check/issue-62879-1.rs:6:12
   |
LL |     let _: [u8; std::mem::size_of::<T>()];
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this may fail depending on what value the parameter takes
