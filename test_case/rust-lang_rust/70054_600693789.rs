
error: failed to run custom build command for `profiler_builtins v0.0.0 (/checkout/src/libprofiler_builtins)`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/profiler_builtins-9bb15f3a7adeeb21/build-script-build` (exit code: 1)
--- stdout
TARGET = Some("arm-linux-androideabi")
OPT_LEVEL = Some("3")
HOST = Some("x86_64-unknown-linux-gnu")
CC_arm-linux-androideabi = Some("sccache /android/ndk/arm-14/bin/arm-linux-androideabi-clang")
CFLAGS_arm-linux-androideabi = Some("-ffunction-sections -fdata-sections -fPIC --target=arm-linux-androideabi")
CRATE_CC_NO_DEFAULTS = None
DEBUG = Some("true")
running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/GCDAProfiling.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c"
exit code: 0
running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/InstrProfiling.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfiling.c"
exit code: 0
running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/InstrProfilingBuffer.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfilingBuffer.c"
exit code: 0
running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/InstrProfilingFile.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfilingFile.c"
cargo:warning=In file included from /checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfilingFile.c:27:
cargo:warning=In file included from /android/ndk/arm-14/bin/../sysroot/usr/include/unistd.h:35:
cargo:warning=In file included from /android/ndk/arm-14/bin/../sysroot/usr/include/sys/select.h:36:
cargo:warning=In file included from /android/ndk/arm-14/bin/../sysroot/usr/include/signal.h:193:
cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:71:33: error: use of undeclared identifier 'LONG_BIT'
cargo:warning=  return (int)((local_set[bit / LONG_BIT] >> (bit % LONG_BIT)) & 1);
cargo:warning=                                ^
cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:71:53: error: use of undeclared identifier 'LONG_BIT'
cargo:warning=  return (int)((local_set[bit / LONG_BIT] >> (bit % LONG_BIT)) & 1);
cargo:warning=                                                    ^
cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:82:19: error: use of undeclared identifier 'LONG_BIT'
cargo:warning=  local_set[bit / LONG_BIT] |= 1UL << (bit % LONG_BIT);
cargo:warning=                  ^
cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:82:46: error: use of undeclared identifier 'LONG_BIT'
cargo:warning=  local_set[bit / LONG_BIT] |= 1UL << (bit % LONG_BIT);
cargo:warning=                                             ^
cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:94:19: error: use of undeclared identifier 'LONG_BIT'
cargo:warning=  local_set[bit / LONG_BIT] &= ~(1UL << (bit % LONG_BIT));
cargo:warning=                  ^
cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:94:48: error: use of undeclared identifier 'LONG_BIT'
cargo:warning=  local_set[bit / LONG_BIT] &= ~(1UL << (bit % LONG_BIT));
cargo:warning=                                               ^
cargo:warning=6 errors generated.
exit code: 1

