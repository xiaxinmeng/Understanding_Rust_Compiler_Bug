plain
[01:34:52] ---- [run-pass] run-pass/issue-18804/main.rs stdout ----
[01:34:52] 
[01:34:52] error: compilation failed!
[01:34:52] status: exit code: 1
[01:34:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-18804/main.rs" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.js" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/auxiliary"
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] stderr:
[01:34:52] stderr:
[01:34:52] ------------------------------------------
[01:34:52] error: linking with `emcc` failed: exit code: 1
[01:34:52]   |
[01:34:52]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.main0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.main1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.main2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.main3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.main4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.main5.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/a.crate.allocator.rcgu.o" "-O0" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-18804/main/auxiliary/liblib.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-4fddd6730dd7c9d0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-9fa99c4cae3524ff.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-1bb7fbecd91a9b8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-b1a900f34d61265a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-550b52f6b124054b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-6637a78bd3b23d81.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-0aa862bb1e920029.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-57285b4d1f8ecbdf.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ABORTING_MALLOC=0" "-s" "WASM=0"
[01:34:52]   = note: error: unresolved symbol: FOO
[01:34:52]           Aborting compilation due to previous errors | undefined
[01:34:52]           Traceback (most recent call last):
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:34:52]               emcc.run()
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:34:52]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[01:34:52]               call_emscripten(cmdline)
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[01:34:52]               temp_files.run_and_clean(lambda: main(
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[01:34:52]               return func()
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[01:34:52]               DEBUG=DEBUG,
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[01:34:52]               temp_files=temp_files, DEBUG=DEBUG)
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 98, in emscript
[01:34:52]               glue, forwarded_data = compiler_glue(metadata, settings, libraries, compiler_engine, temp_files, DEBUG)
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 219, in compiler_glue
[01:34:52]               glue, forwarded_data = compile_settings(compiler_engine, settings, libraries, temp_files)
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 531, in compile_settings
[01:34:52]               cwd=path_from_root('src'), error_limit=300)
[01:34:52]             File "/emsdk-portable/emscripten/1.37.13/tools/jsrun.py", line 128, in run_js
[01:34:52]               raise Exception('Expected the command ' + str(command) + ' to finish with return code ' + str(assert_returncode) + ', but it returned with code ' + str(proc.returncode) + ' instead! Output: ' + str(ret)[:error_limit])
[01:34:52]           Exception: Expected the command ['/emsdk-portable/node/4.1.1_64bit/bin/node', '/emsdk-portable/emscripten/1.37.13/src/compiler.js', '/tmp/tmpADDGoj.txt', '/emsdk-portable/emscripten/1.37.13/src/library_pthread_stub.js'] to finish with return code 0, but it returned with code 1 instead! Output: // The Module object: Our interface to the outside world. We import
[01:34:52]           // and export values on it, and do the work to get that through
[01:34:52]           // closure compiler if necessary. There are various ways Module can be used:
[01:34:52]           // 1. Not defined. We create it here
[01:34:52]           // 2. A function parameter, function(Module) { ..gener
[01:34:52] 
[01:34:52] error: aborting due to previous error
[01:34:52] 
[01:34:52] 
---
[01:34:52] 
[01:34:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:34:52] 
[01:34:52] 
[01:34:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/4.1.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:34:52] 
[01:34:52] 
[01:34:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
[01:34:52] Build completed unsuccessfully in 1:30:39
---
travis_time:end:14b721a9:start=1532420570007858860,finish=1532420570014857062,duration=6998202
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d094c24
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b89101e
travis_time:start:1b89101e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10f828b0
$ dmesg | grep -i kill
