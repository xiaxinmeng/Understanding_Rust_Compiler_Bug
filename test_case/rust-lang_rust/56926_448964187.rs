
[00:03:04] error[E0441]: unrecognized platform-specific intrinsic function: `simd_select_bitmask`
[00:03:04]   --> src/libcore/../stdsimd/coresimd/simd_llvm.rs:52:5
[00:03:04]    |
[00:03:04] 52 |     pub fn simd_select_bitmask<M, T>(m: M, a: T, b: T) -> T;
[00:03:04] 
[00:03:04] error: aborting due to previous error
