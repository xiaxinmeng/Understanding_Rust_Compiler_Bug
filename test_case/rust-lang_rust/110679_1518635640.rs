plain
test [assembly] tests/assembly/x86-stack-probes.rs#i686 ... ok

failures:

---- [assembly] tests/assembly/asm/inline-asm-avx.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/assembly/asm/inline-asm-avx.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "-O" "--emit" "asm" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/asm/inline-asm-avx/inline-asm-avx.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/asm/inline-asm-avx/auxiliary" "--crate-type=lib" "--target" "x86_64-unknown-linux-gnu"
stdout: none
error[E0463]: can't find crate for `std`
  |
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: cannot determine resolution for the macro `asm`
  --> /checkout/tests/assembly/asm/inline-asm-avx.rs:18:9
   |
18 |         asm!(
18 |         asm!(
   |         ^^^
   |
   = note: import resolution is stuck, try simplifying macro imports
error[E0635]: unknown feature `portable_simd`
 --> /checkout/tests/assembly/asm/inline-asm-avx.rs:6:12
  |
6 | #![feature(portable_simd)]
---



failures:
    [assembly] tests/assembly/asm/inline-asm-avx.rs
test result: FAILED. 124 passed; 1 failed; 29 ignored; 0 measured; 0 filtered out; finished in 698.74ms

Some tests failed in compiletest suite=assembly mode=assembly host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
Build completed unsuccessfully in 0:18:57
