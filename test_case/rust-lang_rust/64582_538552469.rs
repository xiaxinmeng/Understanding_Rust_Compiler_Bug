diff
- found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
+ found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`

- error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
+ error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
