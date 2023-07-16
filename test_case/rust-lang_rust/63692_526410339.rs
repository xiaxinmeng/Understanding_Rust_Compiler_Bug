plain
2019-08-30T00:37:53.8284846Z ---- [ui] ui/wrapping-int-combinations.rs stdout ----
2019-08-30T00:37:53.8284948Z 
2019-08-30T00:37:53.8285262Z error: test compilation failed although it shouldn't!
2019-08-30T00:37:53.8285361Z status: exit code: 1
2019-08-30T00:37:53.8286255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wrapping-int-combinations.rs" "-Zthreads=1" "--target=asmjs-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.js" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/auxiliary" "-A" "unused"
2019-08-30T00:37:53.8287142Z ------------------------------------------
2019-08-30T00:37:53.8287183Z 
2019-08-30T00:37:53.8287393Z ------------------------------------------
2019-08-30T00:37:53.8287451Z stderr:
2019-08-30T00:37:53.8287451Z stderr:
2019-08-30T00:37:53.8287658Z ------------------------------------------
2019-08-30T00:37:53.8287909Z error: linking with `emcc` failed: exit code: 1
2019-08-30T00:37:53.8287982Z    |
2019-08-30T00:37:53.8292569Z    = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.wrapping_int_combinations.7rcbfp3g-cgu.8.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/a.12ovfgxg83beefwz.rcgu.o" "-O0" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wrapping-int-combinations/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libstd-49535a7c5c4859d3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libpanic_unwind-393ea22b632776cd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libbacktrace-f61f5b3c61062946.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_demangle-53b709c21d53c34d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libhashbrown-a893d73c7fe4ddeb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_alloc-e0a9bd5d300ce570.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libunwind-b1ec596a278c2955.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcfg_if-fcff7fea802f1c8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liblibc-b5e517bb5d845890.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/liballoc-1337ba99f4510b64.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/librustc_std_workspace_core-602251fb44419723.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcore-608b04ebb869497f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib/libcompiler_builtins-8be4a1b4b2a658d3.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ABORTING_MALLOC=0" "-s" "WASM=0"
2019-08-30T00:37:53.8294495Z    = note: WARNING:root:object /tmp/emscripten_temp_EExgNq_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8295066Z            WARNING:root:object /tmp/emscripten_temp_EExgNq_archive_contents/std-49535a7c5c4859d3.std.86ql6qcu-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8295356Z            WARNING:root:object /tmp/emscripten_temp_EExgNq_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8295842Z            WARNING:root:object /tmp/emscripten_temp_EExgNq_archive_contents/std-49535a7c5c4859d3.std.86ql6qcu-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8296275Z            WARNING:root:object /tmp/emscripten_temp_QU8BEy_archive_contents/rust.metadata.bin is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8297278Z            WARNING:root:object /tmp/emscripten_temp_QU8BEy_archive_contents/panic_unwind-393ea22b632776cd.panic_unwind.8rz2y969-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8297586Z            WARNING:root:object /tmp/emscripten_temp_QU8BEy_archive_contents/rust.metadata.bin is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8298108Z            WARNING:root:object /tmp/emscripten_temp_QU8BEy_archive_contents/panic_unwind-393ea22b632776cd.panic_unwind.8rz2y969-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8298273Z            WARNING:root:object /tmp/emscripten_temp_tWTuRs_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8299761Z            WARNING:root:object /tmp/emscripten_temp_tWTuRs_archive_contents/backtrace-f61f5b3c61062946.backtrace.5lzbpqx0-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8299903Z            WARNING:root:object /tmp/emscripten_temp_pWQRzC_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8300845Z            WARNING:root:object /tmp/emscripten_temp_pWQRzC_archive_contents/rustc_demangle-53b709c21d53c34d.rustc_demangle.acj9k7m4-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8301004Z            WARNING:root:object /tmp/emscripten_temp_DDZ4Kc_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8301769Z            WARNING:root:object /tmp/emscripten_temp_DDZ4Kc_archive_contents/hashbrown-a893d73c7fe4ddeb.hashbrown.8pkkn9v1-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8302108Z            WARNING:root:object /tmp/emscripten_temp_uFHlnj_archive_contents/rust.metadata.bin is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8302481Z            WARNING:root:object /tmp/emscripten_temp_uFHlnj_archive_contents/rustc_std_workspace_alloc-e0a9bd5d300ce570.rustc_std_workspace_alloc.aybnnjzv-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8303344Z            WARNING:root:object /tmp/emscripten_temp_okYKDc_archive_contents/rust.metadata.bin is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8303808Z            WARNING:root:object /tmp/emscripten_temp_okYKDc_archive_contents/unwind-b1ec596a278c2955.unwind.2tbmkfnz-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8305137Z            WARNING:root:object /tmp/emscripten_temp_KR6EC__archive_contents/rust.metadata.bin is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8307270Z            WARNING:root:object /tmp/emscripten_temp_KR6EC__archive_contents/cfg_if-fcff7fea802f1c8b.cfg_if.4nt8hix4-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8307435Z            WARNING:root:object /tmp/emscripten_temp_1fyV3y_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8307892Z            WARNING:root:object /tmp/emscripten_temp_1fyV3y_archive_contents/libc-b5e517bb5d845890.libc.9airplvo-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8308023Z            WARNING:root:object /tmp/emscripten_temp_sDGITL_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8308888Z            WARNING:root:object /tmp/emscripten_temp_sDGITL_archive_contents/alloc-1337ba99f4510b64.alloc.djyzo3c5-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8308992Z            WARNING:root:object /tmp/emscripten_temp_sDGITL_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8309333Z            WARNING:root:object /tmp/emscripten_temp_sDGITL_archive_contents/alloc-1337ba99f4510b64.alloc.djyzo3c5-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8309629Z            WARNING:root:object /tmp/emscripten_temp_kwZGCY_archive_contents/rust.metadata.bin is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8310006Z            WARNING:root:object /tmp/emscripten_temp_kwZGCY_archive_contents/rustc_std_workspace_core-602251fb44419723.rustc_std_workspace_core.72hofv3n-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8310137Z            WARNING:root:object /tmp/emscripten_temp_t2F2hK_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8310529Z            WARNING:root:object /tmp/emscripten_temp_t2F2hK_archive_contents/core-608b04ebb869497f.core.dq9ba3xw-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8310656Z            WARNING:root:object /tmp/emscripten_temp_t2F2hK_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8310990Z            WARNING:root:object /tmp/emscripten_temp_t2F2hK_archive_contents/core-608b04ebb869497f.core.dq9ba3xw-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8311105Z            WARNING:root:object /tmp/emscripten_temp_HIQ5sc_archive_contents/rust.metadata.bin is not LLVM bitcode, cannot link
2019-08-30T00:37:53.8311466Z            WARNING:root:object /tmp/emscripten_temp_HIQ5sc_archive_contents/compiler_builtins-8be4a1b4b2a658d3.compiler_builtins.57ss034p-cgu.0.rcgu.bc.z is not valid according to llvm-nm, cannot link
2019-08-30T00:37:53.8311597Z            Unsupported:   %4 = call fastcc i128 @"_ZN4core3num8wrapping94_$LT$impl$u20$core..ops..bit..Shl$LT$usize$GT$$u20$for$u20$core..num..Wrapping$LT$i128$GT$$GT$3shl17h7740533b68d70d37E"(i128 %3, i32 %1)
2019-08-30T00:37:53.8311715Z            LLVM ERROR: Instruction not yet supported for integer types larger than 64 bits
2019-08-30T00:37:53.8311780Z            Traceback (most recent call last):
2019-08-30T00:37:53.8312034Z              File "/emsdk-portable/emscripten/1.38.15/emcc.py", line 3094, in <module>
2019-08-30T00:37:53.8312095Z                sys.exit(run())
2019-08-30T00:37:53.8312338Z              File "/emsdk-portable/emscripten/1.38.15/emcc.py", line 1807, in run
2019-08-30T00:37:53.8312412Z                final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
2019-08-30T00:37:53.8312678Z              File "/emsdk-portable/emscripten/1.38.15/tools/shared.py", line 2250, in emscripten
2019-08-30T00:37:53.8313162Z                emscripten._main(cmdline)
2019-08-30T00:37:53.8313516Z              File "/emsdk-portable/emscripten/1.38.15/emscripten.py", line 2366, in _main
2019-08-30T00:37:53.8313626Z                return temp_files.run_and_clean(lambda: main(
2019-08-30T00:37:53.8313948Z              File "/emsdk-portable/emscripten/1.38.15/tools/tempfiles.py", line 98, in run_and_clean
2019-08-30T00:37:53.8314054Z                return func()
2019-08-30T00:37:53.8314350Z              File "/emsdk-portable/emscripten/1.38.15/emscripten.py", line 2371, in <lambda>
2019-08-30T00:37:53.8314451Z                DEBUG=DEBUG,
2019-08-30T00:37:53.8314739Z              File "/emsdk-portable/emscripten/1.38.15/emscripten.py", line 2297, in main
2019-08-30T00:37:53.8314844Z                temp_files=temp_files, DEBUG=DEBUG)
2019-08-30T00:37:53.8315137Z              File "/emsdk-portable/emscripten/1.38.15/emscripten.py", line 92, in emscript
2019-08-30T00:37:53.8315248Z                backend_output = compile_js(infile, temp_files, DEBUG)
2019-08-30T00:37:53.8315682Z              File "/emsdk-portable/emscripten/1.38.15/emscripten.py", line 121, in compile_js
2019-08-30T00:37:53.8315830Z                jsrun.timeout_run(subprocess.Popen(backend_args, stdout=subprocess.PIPE, universal_newlines=True), note_args=backend_args)
2019-08-30T00:37:53.8316367Z              File "/emsdk-portable/emscripten/1.38.15/tools/jsrun.py", line 24, in timeout_run
2019-08-30T00:37:53.8316648Z                raise Exception('Subprocess "' + ' '.join(note_args) + '" failed with exit code ' + str(proc.returncode) + '!')
2019-08-30T00:37:53.8317163Z            Exception: Subprocess "/emsdk-portable/clang/e1.38.15_64bit/llc /tmp/tmpqH94VP/a.bc -march=js -filetype=asm -o /tmp/tmp5rS_t1.4.js -emscripten-stack-size=5242880 -O0 -emscripten-assertions=1 -emscripten-no-aliasing-function-pointers -emscripten-global-base=8 -enable-emscripten-cpp-exceptions -emscripten-no-exit-runtime" failed with exit code 1!
2019-08-30T00:37:53.8317348Z 
2019-08-30T00:37:53.8317395Z error: aborting due to previous error
2019-08-30T00:37:53.8317448Z 
2019-08-30T00:37:53.8317479Z 
---
2019-08-30T00:37:53.8320781Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-30T00:37:53.8320849Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-30T00:37:53.8320944Z 
2019-08-30T00:37:53.8320971Z 
2019-08-30T00:37:53.8322377Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "ui" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/8.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-30T00:37:53.8322864Z 
2019-08-30T00:37:53.8322890Z 
2019-08-30T00:37:53.8323190Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/ui src/test/run-fail src/libstd src/liballoc src/libcore
2019-08-30T00:37:53.8323270Z Build completed unsuccessfully in 2:03:02
2019-08-30T00:37:53.8323270Z Build completed unsuccessfully in 2:03:02
2019-08-30T00:37:53.8376484Z == clock drift check ==
2019-08-30T00:37:53.8394180Z   local time: Fri Aug 30 00:37:53 UTC 2019
2019-08-30T00:37:53.9994497Z   network time: Fri, 30 Aug 2019 00:37:53 GMT
2019-08-30T00:37:53.9997831Z == end clock drift check ==
2019-08-30T00:37:54.7413928Z ##[error]Bash exited with code '1'.
2019-08-30T00:37:54.7448206Z ##[section]Starting: Upload CPU usage statistics
2019-08-30T00:37:54.7452220Z ==============================================================================
2019-08-30T00:37:54.7452305Z Task         : Bash
2019-08-30T00:37:54.7452370Z Description  : Run a Bash script on macOS, Linux, or Windows
