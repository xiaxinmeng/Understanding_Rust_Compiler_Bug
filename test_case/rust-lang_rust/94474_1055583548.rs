
error: unsupported operation: unimplemented intrinsic: simd_select
  --> /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/../../portable-simd/crates/core_simd/src/select.rs:37:18
   |
37 |         unsafe { intrinsics::simd_select(self.to_int(), true_values, false_values) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unimplemented intrinsic: simd_select
   |
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
           
   = note: inside `core::core_simd::select::<impl std::simd::Mask<i32, 4_usize>>::select::<i32>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/../../portable-simd/crates/core_simd/src/select.rs:37:18
   = note: inside `core::core_simd::ops::<impl std::ops::Div for std::simd::Simd<i32, 4_usize>>::div` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/../../portable-simd/crates/core_simd/src/ops.rs:85:17
note: inside `simd_ops_i32` at $DIR/portable-simd.rs:21:16
  --> $DIR/portable-simd.rs:21:16
   |
21 |     assert_eq!(a / b, i32x4::from_array([10, 5, 3, 2]));
   |                ^^^^^
note: inside `main` at $DIR/portable-simd.rs:50:5
  --> $DIR/portable-simd.rs:50:5
   |
50 |     simd_ops_i32();
   |     ^^^^^^^^^^^^^^
