plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unrecognized platform-specific intrinsic function: `simd_fsqrt`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:72:5
   |
72 |     pub fn simd_fsqrt<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_fsin`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:73:5
   |
73 |     pub fn simd_fsin<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_fcos`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:74:5
   |
74 |     pub fn simd_fcos<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_fexp`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:78:5
   |
78 |     pub fn simd_fexp<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_fexp2`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:79:5
   |
79 |     pub fn simd_fexp2<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_flog10`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:80:5
   |
80 |     pub fn simd_flog10<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_flog2`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:81:5
   |
81 |     pub fn simd_flog2<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_flog`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:82:5
   |
82 |     pub fn simd_flog<T>(a: T) -> T;


error: unrecognized platform-specific intrinsic function: `simd_fma`
  --> library/core/src/../../stdarch/crates/core_arch/src/simd_llvm.rs:85:5
   |
85 |     pub fn simd_fma<T>(a: T, b: T, c: T) -> T;

error: aborting due to 9 previous errors

error: could not compile `core`
