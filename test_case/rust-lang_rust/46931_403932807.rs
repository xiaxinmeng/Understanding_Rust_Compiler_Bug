
$ rustc -V
rustc 1.25.0 (84203cac6 2018-03-25)
$ cargo test --tests --no-default-features
   Compiling serde v1.0.70
   Compiling rand v0.5.3 (file:///home/dhardy/rust/rand)
error[E0599]: no function or associated item named `from_bits` found for type `f32` in the current scope
   --> src/distributions/float.rs:98:22
    |
98  |                 $ty::from_bits(self | exponent_bits)
    |                      ^^^^^^^^^ function or associated item not found in `f32`
...
149 | float_impls! { f32, u32, f32, u32, 23, 127 }
    | -------------------------------------------- in this macro invocation
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
            candidate #1: `use core::num::Float;`

error[E0599]: no function or associated item named `from_bits` found for type `f64` in the current scope
