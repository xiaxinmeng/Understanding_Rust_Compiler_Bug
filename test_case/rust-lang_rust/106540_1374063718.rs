rust
error: incorrect braces around trait bounds
 --> src/lib.rs:1:25
  |
1 | fn foo2_no_space(_: &dyn(Drop + AsRef<str>)) {}
  |                         ^                 ^
  |
help: remove the parentheses
  |
1 - fn foo2_no_space(_: &dyn(Drop + AsRef<str>)) {}
1 + fn foo2_no_space(_: &dynDrop + AsRef<str>) {}
  |
