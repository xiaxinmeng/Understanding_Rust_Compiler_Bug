
error[E0046]: not all trait items implemented, missing: `from_iter`
 --> src/lib.rs:3:1
  |
3 | impl FromIterator<()> for X {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from_iter` in implementation
  |
  = help: implement the missing item: `fn from_iter(_: T) -> Self { unimplemented!() }`
