plain
[RUSTC-TIMING] build_script_build test:false 0.463
error[E0412]: cannot find type `vector_double` in this scope
 --> library/core/src/../../portable-simd/crates/core_simd/src/vendor/powerpc.rs:9:35
  |
9 | from_transmute! { unsafe f64x2 => vector_double }

error[E0412]: cannot find type `vector_signed_long` in this scope
  --> library/core/src/../../portable-simd/crates/core_simd/src/vendor/powerpc.rs:10:35
   |
   |
10 | from_transmute! { unsafe i64x2 => vector_signed_long }
   |                                   ^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `vector_signed_int`
  ::: library/core/src/../../stdarch/crates/core_arch/src/macros.rs:62:9
   |
   |
62 |         pub struct $name($($fields)*);
   |         ------------------------------ similarly named struct `vector_signed_int` defined here

error[E0412]: cannot find type `vector_unsigned_long` in this scope
  --> library/core/src/../../portable-simd/crates/core_simd/src/vendor/powerpc.rs:11:35
   |
11 | from_transmute! { unsafe u64x2 => vector_unsigned_long }
   |                                   ^^^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `vector_unsigned_int`
  ::: library/core/src/../../stdarch/crates/core_arch/src/macros.rs:62:9
   |
   |
62 |         pub struct $name($($fields)*);
   |         ------------------------------ similarly named struct `vector_unsigned_int` defined here
error: unused import: `core::arch::powerpc64::*`
 --> library/core/src/../../portable-simd/crates/core_simd/src/vendor/powerpc.rs:7:5
  |
7 | use core::arch::powerpc64::*;
7 | use core::arch::powerpc64::*;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0412`.
[RUSTC-TIMING] core test:false 6.352
error: could not compile `core` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
