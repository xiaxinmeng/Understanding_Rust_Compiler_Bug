
[01:32:24] failures:
[01:32:24] 
[01:32:24] ---- [run-pass] run-pass/saturating-float-casts.rs stdout ----
[01:32:24] 	
[01:32:24] error: compilation failed!
[01:32:24] status: exit code: 101
[01:32:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/saturating-float-casts.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/saturating-float-casts.stage2-asmjs-unknown-emscripten.js" "-Crpath" "-O" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-Z" "saturating-float-casts" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/saturating-float-casts.stage2-asmjs-unknown-emscripten.aux"
[01:32:24] stdout:
[01:32:24] ------------------------------------------
[01:32:24] 
[01:32:24] ------------------------------------------
[01:32:24] stderr:
[01:32:24] ------------------------------------------
[01:32:24] error: linking with `emcc` failed: exit code: 1
[01:32:24]   |
[01:32:24]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/saturating-float-casts.saturating_float_casts0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/saturating-float-casts.stage2-asmjs-unknown-emscripten.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/saturating-float-casts.crate.allocator.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/saturating-float-casts.stage2-asmjs-unknown-emscripten.aux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libtest-183e233b7c075ec2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libterm-08fb69f2f719b59c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libgetopts-4f65f6654127877d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-2b35d1b36dfa7e3a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-5bef2c52ac19d279.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-3830ae5679a0b739.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-9ff2c67f40fe5858.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-e1c7eeeb00260d82.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-bad119b0f880de53.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd_unicode-c87147d9878a34a6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librand-efb9ae917b0ef713.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-f61c023fb8fcfea8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-709204c2e3fe5ec9.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1"
[01:32:24]   = note: Unsupported:   %1826 = fptosi float %1825 to i128
[01:32:24]           LLVM ERROR: Instruction not yet supported for integer types larger than 64 bits
[01:32:24]           Traceback (most recent call last):
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:32:24]               emcc.run()
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:32:24]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[01:32:24]               call_emscripten(cmdline)
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[01:32:24]               temp_files.run_and_clean(lambda: main(
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[01:32:24]               return func()
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[01:32:24]               DEBUG=DEBUG,
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[01:32:24]               temp_files=temp_files, DEBUG=DEBUG)
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 93, in emscript
[01:32:24]               backend_output = compile_js(infile, settings, temp_files, DEBUG)
[01:32:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 127, in compile_js
[01:32:24]               backend_output = open(temp_js).read()
[01:32:24]           IOError: [Errno 2] No such file or directory: '/tmp/tmptj926K.4.js'
[01:32:24]           
[01:32:24] 
[01:32:24] error: aborting due to previous error
[01:32:24] 
[01:32:24] 
[01:32:24] ------------------------------------------
[01:32:24] 
[01:32:24] thread '[run-pass] run-pass/saturating-float-casts.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2498:8
[01:32:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:32:24] 
[01:32:24] 
[01:32:24] failures:
[01:32:24]     [run-pass] run-pass/saturating-float-casts.rs
[01:32:24] 
[01:32:24] test result: [31mFAILED(B[m. 2678 passed; 1 failed; 144 ignored; 0 measured; 0 filtered out
