bash
$ rustc ./simd_shuffle_promotion_problem.rs && ./simd_shuffle_promotion_problem
$ rustc -Zmir-opt-level=2 ./simd_shuffle_promotion_problem.rs && ./simd_shuffle_promotion_problem
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Simd2(10, 11)`,
 right: `Simd2(10, 13)`', ./simd_shuffle_promotion_problem.rs:28:5
