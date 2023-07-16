rust
  // export MATMUL_SGEMM_NC=65528 MATMUL_SGEMM_KC=16384 MATMUL_SGEMM_MC=8
  // --target i686-unknown-linux-gnu
  // features = ["constconf"]
  // overflow-checks = false
  assert!(is_x86_feature_detected!("fma"));
  let size = usize::MAX / 256 + 1;
  // SAFETY:
  // - `a` points to a matrix of size 8 x 16384 with strides (0, 0).
  // - `b` points to a matrix of size 16384 x 65528 with strides (0, 0).
  // - `c` points to a matrix of size 8 x 65528 with strides (65528, 1).
  unsafe {
      matrixmultiply::sgemm(
          8,
          16384,
          65528,
          0.0,
          &0.0,
          0,
          0,
          &0.0,
          0,
          0,
          0.0,
          [0.0; 524224].as_mut_ptr(),
          65528,
          1,
      );
  }
  // calls alloc::alloc() with size 0
  // at matrixmultiply::aligned_alloc::Alloc::<f32>::new()
  