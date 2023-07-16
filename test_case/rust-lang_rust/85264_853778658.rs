 
error[E0080]: evaluation of `core::core_arch::macros::ValidateConstImm::<51_i32, 0_i32, 15_i32>::VALID` failed
 --> /home/lqd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/macros.rs:8:9
  |
8 |         assert!(IMM >= MIN && IMM <= MAX, "IMM value not in expected range");
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'IMM value not in expected range', /rustc/dbe459dedd33470f2cb28101157de316caaffa66/library/core/src/../../stdarch/crates/core_arch/src/macros.rs:8:9
  |
  = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

note: the above error was encountered while instantiating `fn std::arch::x86_64::_mm_blend_ps::<51_i32>`
    --> /home/lqd/.cargo/registry/src/github.com-1ecc6299db9ec823/rustfft-5.0.1/src/avx/avx_vector.rs:1314:23
     |
1314 |         let blended = _mm_blend_ps(rows[0], rows[2], 0x33);
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
