
[02:22:18] failures:
[02:22:18] 
[02:22:18] ---- [mir-opt] mir-opt/lower_128bit_debug_test.rs stdout ----
[02:22:18] 	
[02:22:18] error: compilation failed!
[02:22:18] status: exit code: 101
[02:22:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_debug_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--target=asmjs-unknown-emscripten" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test.stage2-asmjs-unknown-emscripten.js" "-Crpath" "-O" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-Z" "lower_128bit_ops" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test.stage2-asmjs-unknown-emscripten.aux"
[02:22:18] stdout:
[02:22:18] ------------------------------------------
[02:22:18] 
[02:22:18] ------------------------------------------
[02:22:18] stderr:
[02:22:18] ------------------------------------------
[02:22:18] error: linking with `emcc` failed: exit code: 1
[02:22:18]   |
[02:22:18]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test.lower_128bit_debug_test0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test.stage2-asmjs-unknown-emscripten.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test.crate.allocator.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test.stage2-asmjs-unknown-emscripten.aux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-a45336ea09f9320b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-d795c34fd7eb72c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-0f4b95d88e53e455.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-6d9370e956155877.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-f874e85b0fc1f5b8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-c7758873e5bedec1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd_unicode-c9b8ae6d904bd621.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-c80c8268f013635d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-6389cf691b0a8db6.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1"
[02:22:18]   = note: Unsupported:   call void @_ZN17compiler_builtins3int6addsub14rust_i128_addo17h7c71827c75f5a0e2E({ i128, i1 }* nocapture nonnull %9, i128 -222, i128 1)
[02:22:18]           LLVM ERROR: Instruction not yet supported for integer types larger than 64 bits
[02:22:18]           Traceback (most recent call last):
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[02:22:18]               emcc.run()
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[02:22:18]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[02:22:18]               call_emscripten(cmdline)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[02:22:18]               temp_files.run_and_clean(lambda: main(
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[02:22:18]               return func()
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[02:22:18]               DEBUG=DEBUG,
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[02:22:18]               temp_files=temp_files, DEBUG=DEBUG)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 93, in emscript
[02:22:18]               backend_output = compile_js(infile, settings, temp_files, DEBUG)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 127, in compile_js
[02:22:18]               backend_output = open(temp_js).read()
[02:22:18]           IOError: [Errno 2] No such file or directory: '/tmp/tmpV98C6G.4.js'
[02:22:18]           
[02:22:18] 
[02:22:18] error: aborting due to previous error
[02:22:18] 
[02:22:18] 
[02:22:18] ------------------------------------------
[02:22:18] 
[02:22:18] thread '[mir-opt] mir-opt/lower_128bit_debug_test.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2570:8
[02:22:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:22:18] 
[02:22:18] ---- [mir-opt] mir-opt/lower_128bit_test.rs stdout ----
[02:22:18] 	
[02:22:18] error: compilation failed!
[02:22:18] status: exit code: 101
[02:22:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--target=asmjs-unknown-emscripten" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test.stage2-asmjs-unknown-emscripten.js" "-Crpath" "-O" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-Z" "lower_128bit_ops" "-C" "debug_assertions=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test.stage2-asmjs-unknown-emscripten.aux"
[02:22:18] stdout:
[02:22:18] ------------------------------------------
[02:22:18] 
[02:22:18] ------------------------------------------
[02:22:18] stderr:
[02:22:18] ------------------------------------------
[02:22:18] error: linking with `emcc` failed: exit code: 1
[02:22:18]   |
[02:22:18]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test.lower_128bit_test0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test.stage2-asmjs-unknown-emscripten.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test.crate.allocator.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test.stage2-asmjs-unknown-emscripten.aux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-a45336ea09f9320b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-d795c34fd7eb72c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-0f4b95d88e53e455.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-6d9370e956155877.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-f874e85b0fc1f5b8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-c7758873e5bedec1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd_unicode-c9b8ae6d904bd621.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-c80c8268f013635d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-6389cf691b0a8db6.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1"
[02:22:18]   = note: Unsupported:   %1 = tail call i128 @_ZN17compiler_builtins3int6addsub13rust_i128_add17h64fc217d5594a832E(i128 -222, i128 1)
[02:22:18]           LLVM ERROR: Instruction not yet supported for integer types larger than 64 bits
[02:22:18]           Traceback (most recent call last):
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[02:22:18]               emcc.run()
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[02:22:18]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[02:22:18]               call_emscripten(cmdline)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[02:22:18]               temp_files.run_and_clean(lambda: main(
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[02:22:18]               return func()
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[02:22:18]               DEBUG=DEBUG,
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[02:22:18]               temp_files=temp_files, DEBUG=DEBUG)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 93, in emscript
[02:22:18]               backend_output = compile_js(infile, settings, temp_files, DEBUG)
[02:22:18]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 127, in compile_js
[02:22:18]               backend_output = open(temp_js).read()
[02:22:18]           IOError: [Errno 2] No such file or directory: '/tmp/tmpQYU40u.4.js'
[02:22:18]           
[02:22:18] 
[02:22:18] error: aborting due to previous error
[02:22:18] 
[02:22:18] 
[02:22:18] ------------------------------------------
[02:22:18] 
[02:22:18] thread '[mir-opt] mir-opt/lower_128bit_test.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2570:8
[02:22:18] 
[02:22:18] 
[02:22:18] failures:
[02:22:18]     [mir-opt] mir-opt/lower_128bit_debug_test.rs
[02:22:18]     [mir-opt] mir-opt/lower_128bit_test.rs
