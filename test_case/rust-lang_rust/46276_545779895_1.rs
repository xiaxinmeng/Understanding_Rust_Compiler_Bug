
error[E0308]: mismatched types
 --> src/lib.rs:3:16
  |
3 |         item = 1;
  |                ^ expected &mut i16, found integer
  |
  = note: expected type `&mut i16`
             found type `{integer}`
help: consider dereferencing here to assign to the mutable borrowed piece of memory
  |
3 |         *item = 1;
  |         ^^^^^
