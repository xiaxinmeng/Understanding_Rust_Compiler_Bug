
[01:37:33] ---- [run-pass] run-pass/smallest-hello-world.rs stdout ----
[01:37:33] 	
[01:37:33] error: compilation failed!
[01:37:33] status: exit code: 101
[01:37:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/smallest-hello-world.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/smallest-hello-world.stage2-asmjs-unknown-emscripten.js" "-Crpath" "-O" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/smallest-hello-world.stage2-asmjs-unknown-emscripten.run-pass.libaux"
[01:37:33] stdout:
[01:37:33] ------------------------------------------
[01:37:33] 
[01:37:33] ------------------------------------------
[01:37:33] stderr:
[01:37:33] ------------------------------------------
[01:37:33] error: linking with `emcc` failed: exit code: 1
[01:37:33]   |
[01:37:33]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/smallest-hello-world.smallest_hello_world0.rust-cgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/smallest-hello-world.stage2-asmjs-unknown-emscripten.js" "-s" "EXPORTED_FUNCTIONS=[\"___rg_shrink_in_place\",\"___rg_alloc\",\"___rg_oom\",\"___rg_realloc\",\"___rg_alloc_zeroed\",\"_rust_eh_personality\",\"___rg_usable_size\",\"___rg_alloc_excess\",\"___rg_grow_in_place\",\"___rg_dealloc\",\"___rg_realloc_excess\",\"_main\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/smallest-hello-world.crate.allocator.rust-cgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/smallest-hello-world.stage2-asmjs-unknown-emscripten.run-pass.libaux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-67992b04c9027e70.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-c15d9e20191e711b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd_unicode-b3230a4723442795.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-bdd38e4620ffab1c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-3181dd9e46400ebd.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1"
[01:37:33]   = note: error: unresolved symbol: rust_begin_unwind
[01:37:33]           Aborting compilation due to previous errors | undefined
[01:37:33]           Traceback (most recent call last):
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:37:33]               emcc.run()
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:37:33]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[01:37:33]               call_emscripten(cmdline)
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[01:37:33]               temp_files.run_and_clean(lambda: main(
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[01:37:33]               return func()
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[01:37:33]               DEBUG=DEBUG,
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[01:37:33]               temp_files=temp_files, DEBUG=DEBUG)
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 98, in emscript
[01:37:33]               glue, forwarded_data = compiler_glue(metadata, settings, libraries, compiler_engine, temp_files, DEBUG)
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 219, in compiler_glue
[01:37:33]               glue, forwarded_data = compile_settings(compiler_engine, settings, libraries, temp_files)
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 531, in compile_settings
[01:37:33]               cwd=path_from_root('src'), error_limit=300)
[01:37:33]             File "/emsdk-portable/emscripten/1.37.13/tools/jsrun.py", line 128, in run_js
[01:37:33]               raise Exception('Expected the command ' + str(command) + ' to finish with return code ' + str(assert_returncode) + ', but it returned with code ' + str(proc.returncode) + ' instead! Output: ' + str(ret)[:error_limit])
[01:37:33]           Exception: Expected the command ['/emsdk-portable/node/4.1.1_64bit/bin/node', '/emsdk-portable/emscripten/1.37.13/src/compiler.js', '/tmp/tmp1WwN3A.txt', '/emsdk-portable/emscripten/1.37.13/src/library_pthread_stub.js'] to finish with return code 0, but it returned with code 1 instead! Output: // The Module object: Our interface to the outside world. We import
[01:37:33]           // and export values on it, and do the work to get that through
[01:37:33]           // closure compiler if necessary. There are various ways Module can be used:
[01:37:33]           // 1. Not defined. We create it here
[01:37:33]           // 2. A function parameter, function(Module) { ..gener
[01:37:33]           
[01:37:33] 
[01:37:33] error: aborting due to previous error
[01:37:33] 
[01:37:33] 
[01:37:33] ------------------------------------------
[01:37:33] 
[01:37:33] thread '[run-pass] run-pass/smallest-hello-world.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2425:8
[01:37:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:37:33] 
[01:37:33] 
[01:37:33] failures:
[01:37:33]     [run-pass] run-pass/smallest-hello-world.rs
[01:37:33] 
[01:37:33] test result: FAILED. 2643 passed; 1 failed; 145 ignored; 0 measured; 0 filtered out
