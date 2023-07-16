
error[E0308]: mismatched types
  --> src/lib.rs:36:41
   |
36 |     let a: Option<Checked<{not_one}>> = None;
   |                                         ^^^^ expected `Scalar(AllocId(1).0x0) : fn(usize) -> bool`, found `Scalar(AllocId(1).0x0) : fn(usize) -> bool`
   |
   = note: expected type `std::option::Option<Checked<Scalar(AllocId(1).0x0) : fn(usize) -> bool>>`
              found type `std::option::Option<_>`

error[E0308]: mismatched types
  --> src/lib.rs:36:9
   |
36 |     let a: Option<Checked<{not_one}>> = None;
   |         ^ expected `Scalar(AllocId(1).0x0) : fn(usize) -> bool`, found `Scalar(AllocId(1).0x0) : fn(usize) -> bool`
   |
   = note: expected type `std::option::Option<Checked<Scalar(AllocId(1).0x0) : fn(usize) -> bool>>`
              found type `_`
