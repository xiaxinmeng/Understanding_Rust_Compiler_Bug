plain
   Compiling std v0.0.0 (/checkout/library/std)
error: unknown arm target feature: crypto
  --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:62:17
   |
31 | /          macro_rules! $macro_name {
33 | |                  ($feature_lit) => {
33 | |                  ($feature_lit) => {
34 | |                      $crate::detect_feature!($feature, $feature_lit $(: $($target_feature_lit),*)?)
62 | |/                 compile_error!(
63 | ||                     concat!(
63 | ||                     concat!(
64 | ||                         concat!("unknown ", stringify!($target)),
65 | ||                         concat!(" target feature: ", $t)
67 | ||                 )
   | ||_________________^
68 | |              };
69 | |          }
69 | |          }
   | |__________- in this expansion of `is_arm_feature_detected!`
  ::: library/std/tests/run-time-detect.rs:19:28
   |
   |
19 |        println!("crypto: {}", is_arm_feature_detected!("crypto"));

[RUSTC-TIMING] run_time_detect test:true 0.046
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
