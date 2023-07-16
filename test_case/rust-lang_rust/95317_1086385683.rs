plain
    |
32  | impl f32 {
    |      ^^^
    |
    = help: consider moving this inherent impl into `core` if possible
help: alternatively add `#[rustc_allow_incoherent_impl]` to the relevant impl items
    |
    |
119 | /     pub fn round_ties_even(self) -> f32 {
120 | |         unsafe { intrinsics::roundevenf32(self) }
    | |_____^

error[E0390]: cannot define inherent `impl` for primitive types outside of `core`
   --> library/std/src/f64.rs:32:6
   --> library/std/src/f64.rs:32:6
    |
32  | impl f64 {
    |      ^^^
    |
    = help: consider moving this inherent impl into `core` if possible
help: alternatively add `#[rustc_allow_incoherent_impl]` to the relevant impl items
    |
    |
119 | /     pub fn round_ties_even(self) -> f64 {
120 | |         unsafe { intrinsics::roundevenf64(self) }
    | |_____^


error[E0599]: no method named `trunc` found for type `f32` in the current scope
    |
    |
162 |         self - self.trunc()
    |                     ^^^^^ method not found in `f32`

error[E0599]: no method named `copysign` found for type `f32` in the current scope
    |
    |
211 |         if self.is_nan() { Self::NAN } else { 1.0_f32.copysign(self) }
    |                                                       ^^^^^^^^ method not found in `f32`

error[E0599]: no method named `trunc` found for type `f32` in the current scope
    |
    |
291 |         let q = (self / rhs).trunc();
    |                              ^^^^^ method not found in `f32`

error[E0599]: no method named `abs` found for type `f32` in the current scope
    |
    |
327 |         if r < 0.0 { r + rhs.abs() } else { r }
    |                              ^^^ method not found in `f32`

error[E0599]: no method named `ln` found for type `f32` in the current scope
    |
    |
478 |         self.ln() / base.ln()
    |              ^^ method not found in `f32`

error[E0599]: no method named `ln` found for type `f32` in the current scope
    |
    |
478 |         self.ln() / base.ln()
    |                          ^^ method not found in `f32`

error[E0599]: no method named `sin` found for type `f32` in the current scope
    |
    |
775 |         (self.sin(), self.cos())
    |               ^^^ help: there is an associated function with a similar name: `min`

error[E0599]: no method named `cos` found for type `f32` in the current scope
    |
    |
775 |         (self.sin(), self.cos())
    |                           ^^^ method not found in `f32`

error[E0599]: no method named `abs` found for type `f32` in the current scope
    |
    |
908 |         (self.abs() + ((self * self) + 1.0).sqrt()).ln().copysign(self)
    |               ^^^ method not found in `f32`

error[E0599]: no method named `sqrt` found for type `f32` in the current scope
    |
    |
908 |         (self.abs() + ((self * self) + 1.0).sqrt()).ln().copysign(self)
    |                                             ^^^^ method not found in `f32`

error[E0599]: no method named `sqrt` found for type `f32` in the current scope
    |
    |
928 |         if self < 1.0 { Self::NAN } else { (self + ((self * self) - 1.0).sqrt()).ln() }
    |                                                                          ^^^^ method not found in `f32`

error[E0599]: no method named `ln` found for type `f32` in the current scope
    |
    |
928 |         if self < 1.0 { Self::NAN } else { (self + ((self * self) - 1.0).sqrt()).ln() }
    |                                                                                  ^^ method not found in `f32`

error[E0599]: no method named `ln_1p` found for type `f32` in the current scope
    |
    |
948 |         0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    |                                             ^^^^^ method not found in `f32`

error[E0599]: no method named `trunc` found for type `f64` in the current scope
    |
    |
162 |         self - self.trunc()
    |                     ^^^^^ method not found in `f64`

error[E0599]: no method named `copysign` found for type `f64` in the current scope
    |
    |
211 |         if self.is_nan() { Self::NAN } else { 1.0_f64.copysign(self) }
    |                                                       ^^^^^^^^ method not found in `f64`

error[E0599]: no method named `trunc` found for type `f64` in the current scope
    |
    |
291 |         let q = (self / rhs).trunc();
    |                              ^^^^^ method not found in `f64`

error[E0599]: no method named `abs` found for type `f64` in the current scope
    |
    |
327 |         if r < 0.0 { r + rhs.abs() } else { r }
    |                              ^^^ method not found in `f64`

error[E0599]: no method named `log_wrapper` found for type `f64` in the current scope
    |
    |
454 |         self.log_wrapper(|n| unsafe { intrinsics::logf64(n) })
    |              ^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `ln` found for type `f64` in the current scope
    |
    |
478 |         self.ln() / base.ln()
    |              ^^ method not found in `f64`

error[E0599]: no method named `ln` found for type `f64` in the current scope
    |
    |
478 |         self.ln() / base.ln()
    |                          ^^ method not found in `f64`

error[E0599]: no method named `log_wrapper` found for type `f64` in the current scope
    |
    |
498 |         self.log_wrapper(|n| {
    |              ^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `log_wrapper` found for type `f64` in the current scope
    |
    |
523 |         self.log_wrapper(|n| unsafe { intrinsics::log10f64(n) })
    |              ^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `sin` found for type `f64` in the current scope
    |
    |
777 |         (self.sin(), self.cos())
    |               ^^^ help: there is an associated function with a similar name: `min`

error[E0599]: no method named `cos` found for type `f64` in the current scope
    |
    |
777 |         (self.sin(), self.cos())
    |                           ^^^ method not found in `f64`

error[E0599]: no method named `abs` found for type `f64` in the current scope
    |
    |
910 |         (self.abs() + ((self * self) + 1.0).sqrt()).ln().copysign(self)
    |               ^^^ method not found in `f64`

error[E0599]: no method named `sqrt` found for type `f64` in the current scope
    |
    |
910 |         (self.abs() + ((self * self) + 1.0).sqrt()).ln().copysign(self)
    |                                             ^^^^ method not found in `f64`

error[E0599]: no method named `sqrt` found for type `f64` in the current scope
    |
    |
930 |         if self < 1.0 { Self::NAN } else { (self + ((self * self) - 1.0).sqrt()).ln() }
    |                                                                          ^^^^ method not found in `f64`

error[E0599]: no method named `ln` found for type `f64` in the current scope
    |
    |
930 |         if self < 1.0 { Self::NAN } else { (self + ((self * self) - 1.0).sqrt()).ln() }
    |                                                                                  ^^ method not found in `f64`

error[E0599]: no method named `ln_1p` found for type `f64` in the current scope
    |
    |
950 |         0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    |                                             ^^^^^ method not found in `f64`
Some errors have detailed explanations: E0390, E0599.
For more information about an error, try `rustc --explain E0390`.
error: could not compile `std` due to 31 previous errors
Build completed unsuccessfully in 0:04:05
