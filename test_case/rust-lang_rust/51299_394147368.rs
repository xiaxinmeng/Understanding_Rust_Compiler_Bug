plain
[01:40:05] ---- [run-pass] run-pass/const-endianess.rs stdout ----
[01:40:05] 
[01:40:05] error: compilation failed!
[01:40:05] status: exit code: 101
[01:40:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/const-endianess.rs" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.js" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/auxiliary"
[01:40:05] ------------------------------------------
[01:40:05] 
[01:40:05] ------------------------------------------
[01:40:05] stderr:
[01:40:05] stderr:
[01:40:05] ------------------------------------------
[01:40:05] error: linking with `emcc` failed: exit code: 1
[01:40:05]   |
[01:40:05]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.const_endianess8.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/a.crate.allocator.rcgu.o" "-O0" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-endianess/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libtest-818ff740b2707f00.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libterm-b839a5796ea44350.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libgetopts-51ccaccdb7d8163e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-054b1990e7d4c7b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-339c04ae4132d76c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-e8ee297dbe25844d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-5e948e8e93b206b3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-8a689bacec06a191.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-057d09441be39ab5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-b8b7fb254e56b0ab.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-371c588c1a3805de.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ABORTING_MALLOC=0"
[01:40:05]   = note: Unsupported:   %99 = call i128 @_ZN4test9black_box17ha1518cd2940bf411E(i128 999999)
[01:40:05]           LLVM ERROR: Instruction not yet supported for integer types larger than 64 bits
[01:40:05]           Traceback (most recent call last):
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:40:05]               emcc.run()
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:40:05]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[01:40:05]               call_emscripten(cmdline)
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[01:40:05]               temp_files.run_and_clean(lambda: main(
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[01:40:05]               return func()
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[01:40:05]               DEBUG=DEBUG,
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[01:40:05]               temp_files=temp_files, DEBUG=DEBUG)
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 93, in emscript
[01:40:05]               backend_output = compile_js(infile, settings, temp_files, DEBUG)
[01:40:05]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 127, in compile_js
[01:40:05]               backend_output = open(temp_js).read()
[01:40:05]           IOError: [Errno 2] No such file or directory: '/tmp/tmpJXtuSk.4.js'
[01:40:05] 
[01:40:05] error: aborting due to previous error
[01:40:05] 
[01:40:05] 
---
[01:40:05] 
[01:40:05] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:40:05] 
[01:40:05] 
[01:40:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/4.1.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:40:05] 
[01:40:05] 
[01:40:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
[01:40:05] Build completed unsuccessfully in 1:36:01
