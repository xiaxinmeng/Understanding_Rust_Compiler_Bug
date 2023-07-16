
[01:34:23] failures:
[01:34:23] 
[01:34:23] ---- [run-pass] run-pass/simd-intrinsic-generic-select.rs stdout ----
[01:34:23] 	
[01:34:23] error: compilation failed!
[01:34:23] status: exit code: 101
[01:34:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/simd-intrinsic-generic-select.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.stage2-asmjs-unknown-emscripten.js" "-Crpath" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.stage2-asmjs-unknown-emscripten.aux"
[01:34:23] stdout:
[01:34:23] ------------------------------------------
[01:34:23] 
[01:34:23] ------------------------------------------
[01:34:23] stderr:
[01:34:23] ------------------------------------------
[01:34:23] warning: type `u32x4` should have a camel case name such as `U32x4`
[01:34:23]   --> /checkout/src/test/run-pass/simd-intrinsic-generic-select.rs:22:1
[01:34:23]    |
[01:34:23] 22 | struct u32x4(pub u32, pub u32, pub u32, pub u32);
[01:34:23]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:23]    |
[01:34:23]    = note: #[warn(non_camel_case_types)] on by default
[01:34:23] 
[01:34:23] warning: type `f32x4` should have a camel case name such as `F32x4`
[01:34:23]   --> /checkout/src/test/run-pass/simd-intrinsic-generic-select.rs:26:1
[01:34:23]    |
[01:34:23] 26 | struct f32x4(pub f32, pub f32, pub f32, pub f32);
[01:34:23]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:23] 
[01:34:23] warning: type `b8x4` should have a camel case name such as `B8x4`
[01:34:23]   --> /checkout/src/test/run-pass/simd-intrinsic-generic-select.rs:30:1
[01:34:23]    |
[01:34:23] 30 | struct b8x4(pub i8, pub i8, pub i8, pub i8);
[01:34:23]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:23] 
[01:34:23] error: linking with `emcc` failed: exit code: 1
[01:34:23]   |
[01:34:23]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.simd_intrinsic_generic_select0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.simd_intrinsic_generic_select1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.simd_intrinsic_generic_select2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.simd_intrinsic_generic_select3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.simd_intrinsic_generic_select4.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.stage2-asmjs-unknown-emscripten.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.crate.allocator.rcgu.o" "-O0" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-select.stage2-asmjs-unknown-emscripten.aux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-7beb25be610b1dc7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-3120ca6f0abf7a1c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-aa0e4e666207dc49.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-46e5367b25f6a1bd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-70db4fd830eb0511.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-2213efc3149321df.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd_unicode-f6f336308541ac07.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-6db47a0e2830c329.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-8d5a0f793c60ffb6.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ABORTING_MALLOC=0"
[01:34:23]   = note:   %245 = trunc <4 x i8> %242 to <4 x i1>
[01:34:23]           LLVM ERROR: invalid vector instr
[01:34:23]           Traceback (most recent call last):
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:34:23]               emcc.run()
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:34:23]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[01:34:23]               call_emscripten(cmdline)
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[01:34:23]               temp_files.run_and_clean(lambda: main(
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[01:34:23]               return func()
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[01:34:23]               DEBUG=DEBUG,
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[01:34:23]               temp_files=temp_files, DEBUG=DEBUG)
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 93, in emscript
[01:34:23]               backend_output = compile_js(infile, settings, temp_files, DEBUG)
[01:34:23]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 127, in compile_js
[01:34:23]               backend_output = open(temp_js).read()
[01:34:23]           IOError: [Errno 2] No such file or directory: '/tmp/tmpukKlpP.4.js'
[01:34:23]           
[01:34:23] 
[01:34:23] error: aborting due to previous error
[01:34:23] 
[01:34:23] 
[01:34:23] ------------------------------------------
[01:34:23] 
[01:34:23] thread '[run-pass] run-pass/simd-intrinsic-generic-select.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[01:34:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:34:23] 
[01:34:23] 
[01:34:23] failures:
[01:34:23]     [run-pass] run-pass/simd-intrinsic-generic-select.rs
