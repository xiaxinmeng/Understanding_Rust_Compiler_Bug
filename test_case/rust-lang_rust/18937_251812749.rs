 rust
lunch-box. rustc --stage1 region-unrelated.rs
error[E0276]: impl has stricter requirements than trait
  --> region-unrelated.rs:21:5
   |
16 |     fn foo() where T: 'a;
   |     --------------------- definition of `foo` from trait
...
21 |     fn foo() where V: 'a { }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `V: 'a`

error: aborting due to previous error
