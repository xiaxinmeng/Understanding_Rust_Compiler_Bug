
error[E0658]: use of unstable library feature 'stdsimd'
 --> src/main.rs:2:8
  |
2 |     if std::arch::is_aarch64_feature_detected!("paca") {
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
  = help: add `#![feature(stdsimd)]` to the crate attributes to enable

error:
               is_aarch64_feature_detected can only be used on AArch64 targets.
               You can prevent it from being used in other architectures by
               guarding it behind a cfg(target_arch) as follows:

                   #[cfg(target_arch = "aarch64")] {
                       if is_aarch64_feature_detected(...) { ... }
                   }

 --> src/main.rs:2:8
  |
2 |     if std::arch::is_aarch64_feature_detected!("paca") {
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the macro `std::arch::is_aarch64_feature_detected` (in Nightly builds, run with -Z macro-backtrace for more info)
