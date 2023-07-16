plain
[00:04:06]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:07]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:04:08]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:09]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:11] error[E0599]: no method named `round` found for type `f64` in the current scope
[00:04:11]     |
[00:04:11]     |
[00:04:11] 527 |         let nanos_u128 = nanos_f64.round() as u128;
[00:04:11] 
[00:04:11] 
[00:04:11] error[E0599]: no method named `round` found for type `f64` in the current scope
[00:04:11]     |
[00:04:11]     |
[00:04:11] 553 |         let nanos_u128 = nanos_f64.round() as u128;
[00:04:11] 
[00:04:11] 
[00:04:11] error[E0599]: no method named `round` found for type `f64` in the current scope
[00:04:11]     |
[00:04:11]     |
[00:04:11] 602 |         let nanos_u128 = nanos_f64.round() as u128;
[00:04:11] 
[00:04:11] error: aborting due to 3 previous errors
[00:04:11] 
[00:04:11] For more information about this error, try `rustc --explain E0599`.
