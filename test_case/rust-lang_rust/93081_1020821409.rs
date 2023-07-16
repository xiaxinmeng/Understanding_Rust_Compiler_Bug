plain
test src/unit.rs - unit::() (line 8) ... ok

failures:

---- src/num/nonzero.rs - num::nonzero::NonZeroU8::is_power_of_two (line 999) stdout ----
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/saturating.rs - num::saturating::Saturating<i8>::count_ones (line 978) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/saturating.rs - num::saturating::Saturating<i8>::count_zeros (line 978) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/saturating.rs - num::saturating::Saturating<u8>::count_zeros (line 978) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/saturating.rs - num::saturating::Saturating<u8>::count_ones (line 978) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/saturating.rs - num::saturating::Saturating<u8>::is_power_of_two (line 1163) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/wrapping.rs - num::wrapping::Wrapping<i8>::count_ones (line 811) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/wrapping.rs - num::wrapping::Wrapping<i8>::count_zeros (line 811) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/wrapping.rs - num::wrapping::Wrapping<u8>::count_zeros (line 811) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/wrapping.rs - num::wrapping::Wrapping<u8>::count_ones (line 811) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!
---- src/num/wrapping.rs - num::wrapping::Wrapping<u8>::is_power_of_two (line 1013) stdout ----
B0 = COPY H0
B0 = COPY H0
unimplemented reg-to-reg copy
UNREACHABLE executed at /checkout/src/llvm-project/llvm/lib/Target/AArch64/AArch64InstrInfo.cpp:3633!

failures:
    src/num/nonzero.rs - num::nonzero::NonZeroU8::is_power_of_two (line 999)
    src/num/saturating.rs - num::saturating::Saturating<i8>::count_ones (line 978)
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "14" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--"


Build completed unsuccessfully in 0:29:45
