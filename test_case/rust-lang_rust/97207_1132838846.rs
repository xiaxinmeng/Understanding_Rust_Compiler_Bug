
   Compiling simd-adler32 v0.3.4
error[E0463]: can't find crate for `std`

error[E0463]: can't find crate for `core`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/simd-adler32-0.3.4/src/imp/avx2.rs:49:7
   |
49 |   use core::arch::x86_64::*;
   |       ^^^^ can't find crate

error[E0463]: can't find crate for `core`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/simd-adler32-0.3.4/src/imp/sse2.rs:49:7
   |
49 |   use core::arch::x86_64::*;
   |       ^^^^ can't find crate

error[E0463]: can't find crate for `core`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/simd-adler32-0.3.4/src/imp/ssse3.rs:49:7
   |
49 |   use core::arch::x86_64::*;
   |       ^^^^ can't find crate

[...]

error: could not compile `simd-adler32` due to 149 previous errors
