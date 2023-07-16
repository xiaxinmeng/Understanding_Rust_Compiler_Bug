plain
   Compiling miniz_oxide v0.4.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.12.3
   Compiling addr2line v0.16.0
error[E0432]: unresolved imports `core::f32::NEG_ZERO`, `core::f32::ZERO`
   |
   |
27 |     MIN_EXP, MIN_POSITIVE, NAN, NEG_INFINITY, NEG_ZERO, RADIX, ZERO,
   |                                               ^^^^^^^^         ^^^^ no `ZERO` in `f32`
   |                                               |
   |                                               no `NEG_ZERO` in `f32`

error[E0432]: unresolved imports `core::f64::NEG_ZERO`, `core::f64::ZERO`
   |
   |
27 |     MIN_EXP, MIN_POSITIVE, NAN, NEG_INFINITY, NEG_ZERO, RADIX, ZERO, 
   |                                               ^^^^^^^^         ^^^^ no `ZERO` in `f64`
   |                                               |
   |                                               no `NEG_ZERO` in `f64`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:19
