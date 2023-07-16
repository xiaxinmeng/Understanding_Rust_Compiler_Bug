
error[[E0106]](https://doc.rust-lang.org/stable/error-index.html#E0106): missing lifetime specifier
 --> src/lib.rs:1:32
  |
1 | fn foo(_: impl Iterator<Item = &u32>) {}
  |                                ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 | fn foo<'a>(_: impl Iterator<Item = &'a u32>) {}
  |       ++++                          ++
