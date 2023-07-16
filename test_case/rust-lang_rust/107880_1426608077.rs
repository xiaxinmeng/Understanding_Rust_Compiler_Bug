rust
error[E0252]: the name `vcvtq_u32_f32` is defined multiple times
  --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm/mod.rs:81:9
   |
73 | pub use crate::core_arch::arm_shared::*;
   |         -------------------------------
   |         |
   |         previous import of the value `vcvtq_u32_f32` here
   |         you can use `as` to change the binding name of the import
...
81 | pub use neon::*;
   |         ^^^^^^^ `vcvtq_u32_f32` reimported here
   |
   = note: `vcvtq_u32_f32` must be defined only once in the value namespace of this module
