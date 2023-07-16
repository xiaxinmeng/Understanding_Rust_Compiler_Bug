plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.11.0
   Compiling addr2line v0.16.0
error: cannot find attribute `lang_item` in this scope
   |
   |
96 | #[lang_item = "f64_simd_runtime"]


error: cannot find attribute `lang_item` in this scope
   |
   |
32 | #[lang_item = "f32_simd_runtime"]

error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
   --> library/std/src/simd.rs:97:1
    |
    |
97  | / impl<const N: usize> Simd<f64, N> {
98  | |     /// Fused multiply-add.  Computes `(self * a) + b` with only one rounding error,
99  | |     /// yielding a more accurate result than an unfused multiply-add.
...   |
154 | |     }
155 | | }
155 | | }
    | |_^ impl for type defined outside of crate.
    = note: define and implement a trait or new type instead

error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
  --> library/std/src/simd.rs:33:1
  --> library/std/src/simd.rs:33:1
   |
33 | / impl<const N: usize> Simd<f32, N> {
34 | |     /// Fused multiply-add.  Computes `(self * a) + b` with only one rounding error,
35 | |     /// yielding a more accurate result than an unfused multiply-add.
...  |
90 | |     }
91 | | }
91 | | }
   | |_^ impl for type defined outside of crate.
   = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0116`.
error: could not compile `std` due to 4 previous errors
