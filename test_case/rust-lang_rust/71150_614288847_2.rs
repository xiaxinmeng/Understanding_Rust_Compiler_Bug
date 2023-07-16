
error[E0046]: not all trait items implemented, missing: `f`
 --> src\main.rs:2:1
  |
2 | impl my_crate::Tr for S {}
  | ^^^^^^^^^^^^^^^^^^^^^^^ missing `f` in implementation
  |
  = help: implement the missing item: `fn f(&self, &self) { unimplemented!() }`
