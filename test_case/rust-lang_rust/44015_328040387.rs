
[01:50:36] error: linking with `emcc` failed: exit code: 1
[01:50:36]   |
[01:50:36]   = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/collectionstests-a051b7c76aca0939.0.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/collectionstests-a051b7c76aca0939.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"___rdl_shrink_in_place\",\"___rdl_alloc_excess\",\"___rdl_usable_size\",\"___rdl_alloc\",\"___rdl_realloc_excess\",\"___rdl_realloc\",\"___rdl_oom\",\"___rdl_grow_in_place\",\"___rdl_alloc_zeroed\",\"___rdl_dealloc\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/collectionstests-a051b7c76aca0939.crate.allocator.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libtest-7174f69942758dfc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libterm-b41696b9a2882d36.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libgetopts-b40550347d4814f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-5e8ebc384e5dfd82.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-368647e9b9ca3b39.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-eda6aedbad712bc0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc_system-67992b04c9027e70.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-0200a904d71d63a4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-c15d9e20191e711b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd_unicode-b3230a4723442795.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librand-efc56d5e4c2b1ee8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-3181dd9e46400ebd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-fa8533728d55e42b.rlib" "-l" "c" "-Wl,-rpath,$ORIGIN/../lib" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1"
[01:50:36]   = note: LLVM ERROR: Function _ZN4core4hash6Hasher10write_i12817h4a3f787e99e09ab3E has illegal integer argument
[01:50:36]           Traceback (most recent call last):
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:50:36]               emcc.run()
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:50:36]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/tools/shared.py", line 1963, in emscripten
[01:50:36]               call_emscripten(cmdline)
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2190, in _main
[01:50:36]               temp_files.run_and_clean(lambda: main(
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/tools/tempfiles.py", line 78, in run_and_clean
[01:50:36]               return func()
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2195, in <lambda>
[01:50:36]               DEBUG=DEBUG,
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 2095, in main
[01:50:36]               temp_files=temp_files, DEBUG=DEBUG)
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 93, in emscript
[01:50:36]               backend_output = compile_js(infile, settings, temp_files, DEBUG)
[01:50:36]             File "/emsdk-portable/emscripten/1.37.13/emscripten.py", line 127, in compile_js
[01:50:36]               backend_output = open(temp_js).read()
[01:50:36]           IOError: [Errno 2] No such file or directory: '/tmp/tmppyyP2C.4.js'
[01:50:36]           
[01:50:36]
[01:50:36] error: aborting due to previous error
[01:50:36]
[01:50:36] error: Could not compile `alloc`.
[01:50:36] warning: build failed, waiting for other jobs to finish...
[01:51:11] error: build failed
[01:51:11]
[01:51:11]
[01:51:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "asmjs-unknown-emscripten" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std:0.0.0" "-p" "unwind:0.0.0" "-p" "alloc:0.0.0" "-p" "alloc_system:0.0.0" "-p" "collections:0.0.0" "-p" "std_unicode:0.0.0" "-p" "panic_abort:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "rand:0.0.0" "-p" "core:0.0.0" "-p" "libc:0.0.0" "--"
[01:51:11] expected success, got: exit code: 101
[01:51:11]
[01:51:11]
[01:51:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten
[01:51:11] Build completed unsuccessfully in 1:49:01
