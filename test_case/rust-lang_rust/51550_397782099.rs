plain
[01:36:24] ---- [run-pass] run-pass/assert-eq-macro-unsized.rs stdout ----
[01:36:24] 
[01:36:24] error: compilation failed!
[01:36:24] status: exit code: 101
[01:36:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/assert-eq-macro-unsized.rs" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.js" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/auxiliary"
[01:36:24] ------------------------------------------
[01:36:24] 
[01:36:24] ------------------------------------------
[01:36:24] stderr:
[01:36:24] stderr:
[01:36:24] ------------------------------------------
[01:36:24] error: linking with `emcc` failed: exit code: 1
[01:36:24]   |
[01:36:24]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.assert_eq_macro_unsized9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/a.crate.allocator.rcgu.o" "-O0" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/assert-eq-macro-unsized/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-054b1990e7d4c7b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-339c04ae4132d76c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-e8ee297dbe25844d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-5e948e8e93b206b3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-8a689bacec06a191.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-057d09441be39ab5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-b8b7fb254e56b0ab.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-371c588c1a3805de.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ABORTING_MALLOC=0"
[01:36:24]   = note: warning: unexpected number of arguments 2 in call to 'rust_oom', should be 1
[01:36:24]           warning: unexpected number of arguments 2 in call to 'rust_oom', should be 1
[01:36:24]           warning: unexpected number of arguments 2 in call to 'rust_oom', should be 1
[01:36:24]           warning: unexpected number of arguments 2 in call to 'rust_oom', should be 1
[01:36:24]           error: unresolved symbol: _ZN12alloc_system16realloc_fallback38_$LT$impl$u20$alloc_system__System$GT$16realloc_fallback17h6e1ebdc38039a666E
[01:36:24]           Aborting compilation due to previous errors | undefined
[01:36:24]           Traceback (most recent call last):
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:36:24]               emcc.run()
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:36:24]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[01:36:24]               call_emscripten(cmdline)
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[01:36:24]               temp_files.run_and_clean(lambda: main(
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[01:36:24]               return func()
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[01:36:24]               DEBUG=DEBUG,
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[01:36:24]               temp_files=temp_files, DEBUG=DEBUG)
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 98, in emscript
[01:36:24]               glue, forwarded_data = compiler_glue(metadata, settings, libraries, compiler_engine, temp_files, DEBUG)
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 219, in compiler_glue
[01:36:24]               glue, forwarded_data = compile_settings(compiler_engine, settings, libraries, temp_files)
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 531, in compile_settings
[01:36:24]               cwd=path_from_root('src'), error_limit=300)
[01:36:24]             File "/emsdk-portable/emscripten/1.37.13/tools/jsrun.py", line 128, in run_js
[01:36:24]               raise Exception('Expected the command ' + str(command) + ' to finish with return code ' + str(assert_returncode) + ', but it returned with code ' + str(proc.returncode) + ' instead! Output: ' + str(ret)[:error_limit])
[01:36:24]           Exception: Expected the command ['/emsdk-portable/node/4.1.1_64bit/bin/node', '/emsdk-portable/emscripten/1.37.13/src/compiler.js', '/tmp/tmp6zHs_K.txt', '/emsdk-portable/emscripten/1.37.13/src/library_pthread_stub.js'] to finish with return code 0, but it returned with code 1 instead! Output: // The Module object: Our interface to the outside world. We import
[01:36:24]           // and export values on it, and do the work to get that through
[01:36:24]           // closure compiler if necessary. There are various ways Module can be used:
[01:36:24]           // 1. Not defined. We create it here
[01:36:24]           // 2. A function parameter, function(Module) { ..gener
[01:36:24] 
[01:36:24] error: aborting due to previous error
[01:36:24] 
[01:36:24] 
---
[01:36:24] 
[01:36:24] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:36:24] 
[01:36:24] 
[01:36:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/4.1.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:36:24] 
[01:36:24] 
[01:36:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
[01:36:24] Build completed unsuccessfully in 1:32:09
---
travis_time:end:10317a58:start=1529118336625204397,finish=1529118336635108941,duration=9904544
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b65937a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0180224a
$ dmesg | grep -i kill
