plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0599]: no method named `abs` found for struct `Simd` in the current scope
  --> library/core/tests/simd.rs:10:15
   |
10 |     let r = y.abs();
   |
  ::: /checkout/library/core/src/../../portable-simd/crates/core_simd/src/elements/float.rs:31:8
   |
31 |     fn abs(self) -> Self;
