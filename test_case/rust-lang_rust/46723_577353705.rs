
error[E0046]: not all trait items implemented, missing: `fmt`
 --> src/main.rs:3:1
  |
3 | impl Debug for Foo {}
  | ^^^^^^^^^^^^^^^^^^ missing `fmt` in implementation
  |
  = help: implement the missing item: `fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { unimplemented!() }`
