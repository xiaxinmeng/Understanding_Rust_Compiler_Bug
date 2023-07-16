
error[E0428]: the name `test` is defined multiple times
 --> t.rs:5:1
  |
3 | pub fn test() { }
  | ------------- previous definition of the value `test` here
4 |
5 | pub fn test() { }
  | ^^^^^^^^^^^^^ `test` redefined here
  |
  = note: `test` must be defined only once in the value namespace of this module

error[E0255]: the name `test` is defined multiple times
 --> t.rs:3:1
  |
1 | use test;
  |     ---- previous import of the value `test` here
2 |
3 | pub fn test() { }
  | ^^^^^^^^^^^^^ `test` redefined here
  |
  = note: `test` must be defined only once in the value namespace of this module
help: You can use `as` to change the binding name of the import
  |
1 | use test as other_test;
  |     ^^^^^^^^^^^^^^^^^^

warning: unused import: `test`
 --> t.rs:1:5
  |
1 | use test;
  |     ^^^^
  |
  = note: #[warn(unused_imports)] on by default

error[E0601]: `main` function not found in crate `t`
  |
  = note: consider adding a `main` function to `t.rs`

error: aborting due to 3 previous errors

Some errors occurred: E0255, E0428, E0601.
For more information about an error, try `rustc --explain E0255`.
