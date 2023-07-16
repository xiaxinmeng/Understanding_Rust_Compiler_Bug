
error[E0511]: invalid monomorphization of `simd_reduce_mul_ordered` intrinsic: expected return type `i8` (element of input `Simd<i8, 16_usize>`), found `i32`
  --> src\main.rs:37:26
   |
37 |         let a = unsafe { crate::simd_reduce_mul_ordered(crate::Simd::<i8, 16>::from_array(x), 1) };
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
