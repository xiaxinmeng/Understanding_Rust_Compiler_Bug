
error[E0658]: use of unstable library feature 'allocator_api'
  --> crates/<my_crate>/src/main.rs:3:33
  |
3 | pub type Foo = BTreeSet<String, String>;
  |                                 ^^^^^^
  |
   = note: see issue #32838 <https://github.com/rust-lang/rust/issues/32838> for more information

For more information about this error, try `rustc --explain E0658`.
