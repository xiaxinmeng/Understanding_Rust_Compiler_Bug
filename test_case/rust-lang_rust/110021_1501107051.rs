plain
---- [codegen] tests/codegen/intrinsics/transmute.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/intrinsics/transmute.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/intrinsics/transmute" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/intrinsics/transmute/auxiliary" "-O" "-C" "no-prepopulate-passes"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `std::arch::x86_64`
   |
   |
11 | use std::arch::x86_64::{__m128, __m128i, __m256i};
   |                ^^^^^^ could not find `x86_64` in `arch`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
