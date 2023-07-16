rust
warning: struct is never constructed: `Foo`
 --> src/lib.rs:1:1
  |
1 | struct Foo;
  | ^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `foo`
 --> src/lib.rs:3:1
  |
3 | fn foo() {
  | ^^^^^^^^
