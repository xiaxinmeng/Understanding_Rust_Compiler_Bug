
error[E0599]: no function or associated item named `copy` found for struct `S` in the current scope
  --> src/main.rs:16:22
   |
8  |     pub struct S;
   |     ------------- function or associated item `copy` not found for this
...
16 |     let f: fn() = S::copy;
   |                      ^^^^ function or associated item not found in `S`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Sealed` defines an item `copy`, perhaps you need to implement it
  --> src/main.rs:3:9
   |
3  |         pub(super) trait Sealed {
   |         ^^^^^^^^^^^^^^^^^^^^^^^
