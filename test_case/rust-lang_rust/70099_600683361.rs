plain
2020-03-18T15:16:57.5832274Z [RUSTC-TIMING] compiler_builtins test:false 2.155
2020-03-18T15:16:57.5982123Z error: failed to run custom build command for `profiler_builtins v0.0.0 (/checkout/src/libprofiler_builtins)`
2020-03-18T15:16:57.5983019Z 
2020-03-18T15:16:57.5983595Z Caused by:
2020-03-18T15:16:57.5984961Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/profiler_builtins-9bb15f3a7adeeb21/build-script-build` (exit code: 1)
2020-03-18T15:16:57.5985789Z --- stdout
2020-03-18T15:16:57.5986267Z TARGET = Some("arm-linux-androideabi")
2020-03-18T15:16:57.5986658Z OPT_LEVEL = Some("3")
2020-03-18T15:16:57.5987163Z HOST = Some("x86_64-unknown-linux-gnu")
2020-03-18T15:16:57.5988906Z CC_arm-linux-androideabi = Some("sccache /android/ndk/arm-14/bin/arm-linux-androideabi-clang")
2020-03-18T15:16:57.5990367Z CFLAGS_arm-linux-androideabi = Some("-ffunction-sections -fdata-sections -fPIC --target=arm-linux-androideabi")
2020-03-18T15:16:57.5990811Z CRATE_CC_NO_DEFAULTS = None
2020-03-18T15:16:57.5991080Z DEBUG = Some("true")
2020-03-18T15:16:57.5993884Z running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/GCDAProfiling.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c"
2020-03-18T15:16:57.5995852Z exit code: 0
2020-03-18T15:16:57.5998484Z running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/InstrProfiling.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfiling.c"
2020-03-18T15:16:57.6000179Z exit code: 0
2020-03-18T15:16:57.6002960Z running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/InstrProfilingBuffer.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfilingBuffer.c"
2020-03-18T15:16:57.6004683Z exit code: 0
2020-03-18T15:16:57.6007346Z running: "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/InstrProfilingFile.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfilingFile.c"
2020-03-18T15:16:57.6009548Z cargo:warning=In file included from /checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfilingFile.c:27:
2020-03-18T15:16:57.6010461Z cargo:warning=In file included from /android/ndk/arm-14/bin/../sysroot/usr/include/unistd.h:35:
2020-03-18T15:16:57.6011322Z cargo:warning=In file included from /android/ndk/arm-14/bin/../sysroot/usr/include/sys/select.h:36:
2020-03-18T15:16:57.6012195Z cargo:warning=In file included from /android/ndk/arm-14/bin/../sysroot/usr/include/signal.h:193:
2020-03-18T15:16:57.6013242Z cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:71:33: error: use of undeclared identifier 'LONG_BIT'
2020-03-18T15:16:57.6013906Z cargo:warning=  return (int)((local_set[bit / LONG_BIT] >> (bit % LONG_BIT)) & 1);
2020-03-18T15:16:57.6014359Z cargo:warning=                                ^
2020-03-18T15:16:57.6015269Z cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:71:53: error: use of undeclared identifier 'LONG_BIT'
2020-03-18T15:16:57.6015943Z cargo:warning=  return (int)((local_set[bit / LONG_BIT] >> (bit % LONG_BIT)) & 1);
2020-03-18T15:16:57.6016558Z cargo:warning=                                                    ^
2020-03-18T15:16:57.6017544Z cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:82:19: error: use of undeclared identifier 'LONG_BIT'
2020-03-18T15:16:57.6018198Z cargo:warning=  local_set[bit / LONG_BIT] |= 1UL << (bit % LONG_BIT);
2020-03-18T15:16:57.6018567Z cargo:warning=                  ^
2020-03-18T15:16:57.6019460Z cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:82:46: error: use of undeclared identifier 'LONG_BIT'
2020-03-18T15:16:57.6020104Z cargo:warning=  local_set[bit / LONG_BIT] |= 1UL << (bit % LONG_BIT);
2020-03-18T15:16:57.6020534Z cargo:warning=                                             ^
2020-03-18T15:16:57.6021479Z cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:94:19: error: use of undeclared identifier 'LONG_BIT'
2020-03-18T15:16:57.6022116Z cargo:warning=  local_set[bit / LONG_BIT] &= ~(1UL << (bit % LONG_BIT));
2020-03-18T15:16:57.6022516Z cargo:warning=                  ^
2020-03-18T15:16:57.6023477Z cargo:warning=/android/ndk/arm-14/bin/../sysroot/usr/include/android/legacy_signal_inlines.h:94:48: error: use of undeclared identifier 'LONG_BIT'
2020-03-18T15:16:57.6024231Z cargo:warning=  local_set[bit / LONG_BIT] &= ~(1UL << (bit % LONG_BIT));
2020-03-18T15:16:57.6024705Z cargo:warning=                                               ^
2020-03-18T15:16:57.6025060Z cargo:warning=6 errors generated.
2020-03-18T15:16:57.6025490Z 
2020-03-18T15:16:57.6025912Z --- stderr
2020-03-18T15:16:57.6026061Z 
2020-03-18T15:16:57.6026181Z 
2020-03-18T15:16:57.6026181Z 
2020-03-18T15:16:57.6030334Z error occurred: Command "sccache" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=arm-linux-androideabi" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=arm-linux-androideabi" "-I" "/checkout/src/llvm-project/compiler-rt/include" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/profiler_builtins-48aac5a25541f1f3/out/InstrProfilingFile.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfilingFile.c" with args "arm-linux-androideabi-clang" did not execute successfully (status code exit code: 1).
2020-03-18T15:16:57.6032410Z 
2020-03-18T15:16:57.6032546Z 
2020-03-18T15:16:57.6033315Z warning: build failed, waiting for other jobs to finish...
2020-03-18T15:16:57.6081724Z error: build failed
---
2020-03-18T15:16:57.6190857Z   local time: Wed Mar 18 15:16:57 UTC 2020
2020-03-18T15:16:58.1770875Z   network time: Wed, 18 Mar 2020 15:16:58 GMT
2020-03-18T15:16:58.1773903Z == end clock drift check ==
2020-03-18T15:17:03.7424348Z 
2020-03-18T15:17:03.7544631Z ##[error]Bash exited with code '1'.
2020-03-18T15:17:03.7633083Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-18T15:17:03.7639527Z ==============================================================================
2020-03-18T15:17:03.7640072Z Task         : Get sources
2020-03-18T15:17:03.7640622Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
