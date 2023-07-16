
error[[E0080]](https://doc.rust-lang.org/stable/error-index.html#E0080): evaluation of `core::core_arch::macros::ValidateConstImm::<51, 0, 15>::VALID` failed
  |
  = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

note: the above error was encountered while instantiating `fn std::arch::x86_64::_mm_blend_ps::<51>`
 --> src/main.rs:7:24
  |
7 |         let _blended = _mm_blend_ps(a, b, 0x33);
  |                        ^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0080`.
error: could not compile `playground` due to previous error
