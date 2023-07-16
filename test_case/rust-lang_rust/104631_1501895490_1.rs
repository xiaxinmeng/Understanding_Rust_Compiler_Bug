
error[E0277]: `AlignedUsize` needs to have the same ABI as a pointer
  --> src/main.rs:11:14
   |
11 |     let _x = AlignedUsize(12) as dyn* Debug;
   |              ^^^^^^^^^^^^^^^^ `AlignedUsize` needs to be a pointer-like type
   |
   = help: the trait `PointerLike` is not implemented for `AlignedUsize`

For more information about this error, try `rustc --explain E0277`.
