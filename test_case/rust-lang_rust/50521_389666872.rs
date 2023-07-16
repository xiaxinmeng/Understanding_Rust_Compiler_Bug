plain
[01:37:12] ..................i..i........................................i.....................................
[01:37:16] ......test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[01:37:51] ......................................i.......................................................
[01:38:12] .......................................................................................ii...........
[01:38:57] ..............................i.................iF..................................................
[01:40:24] ..i.................................................................................................
[01:40:50] ....................................................................................................
[01:41:18] ....................................................................................................
[01:41:44] .i......................................................................................
[01:41:44] .i......................................................................................
[01:41:44] failures:
[01:41:44] 
[01:41:44] ---- [run-pass] run-pass/simd-intrinsic-float-math.rs stdout ----
[01:41:44]  
[01:41:44] error: compilation failed!
[01:41:44] status: exit code: 101
[01:41:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/simd-intrinsic-float-math.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=arm-linux-androideabi" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.stage2-arm-linux-androideabi" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.stage2-arm-linux-androideabi.aux"
[01:41:44] ------------------------------------------
[01:41:44] 
[01:41:44] ------------------------------------------
[01:41:44] stderr:
[01:41:44] stderr:
[01:41:44] ------------------------------------------
[01:41:44] error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit code: 1
[01:41:44]   |
[01:41:44]   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--allow-multiple-definition" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.simd_intrinsic_float_math0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.simd_intrinsic_float_math2-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.stage2-arm-linux-androideabi" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.stage2-arm-linux-androideabi.aux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-l" "std-dd7e1cb952f01bfe" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-93fbfd71748200b0.rlib" "-Wl,-Bdynamic" "-l" "dl" "-l" "log" "-l" "gcc" "-l" "gcc" "-l" "c" "-l" "m" "-Wl,-rpath,$ORIGIN/../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags"
[01:41:44]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs.rcgu.o:simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs:function simd_intrinsic_float_math::main::h3154db08c8edf39b: error: undefined reference to 'log2f'
[01:41:44]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs.rcgu.o:simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs:function simd_intrinsic_float_math::main::h3154db08c8edf39b: error: undefined reference to 'log2f'
[01:41:44]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs.rcgu.o:simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs:function simd_intrinsic_float_math::main::h3154db08c8edf39b: error: undefined reference to 'log2f'
[01:41:44]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-float-math.simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs.rcgu.o:simd_intrinsic_float_math1-317d481089b8c8fe83113de504472633.rs:function simd_intrinsic_float_math::main::h3154db08c8edf39b: error: undefined reference to 'log2f'
[01:41:44]           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
[01:41:44] 
[01:41:44] error: aborting due to previous error
[01:41:44] 
[01:41:44] 
---
[01:41:44] 
[01:41:44] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:41:44] 
[01:41:44] 
[01:41:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "run-pass" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--quiet" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:41:44] 
[01:41:44] 
[01:41:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:41:44] Build completed unsuccessfully in 1:17:54
