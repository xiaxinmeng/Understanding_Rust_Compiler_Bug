plain
[00:52:46] ....................................................................................................
[00:52:50] ....................................................................................................
[00:52:53] ...............i....................................................................................
[00:52:56] ....................................................................................................
[00:52:59] ................................................................iiiiiiiii...........................
[00:53:04] ....................................................................................................
[00:53:08] ....................................................................................................
[00:53:11] ............................................i.......................................................
[00:53:14] ..............................................................................................i.i..i
---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:52] 
[01:22:52] running 190 tests
[01:23:23] .......................................................................................F............
[01:24:17] .........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:24:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:24:58] failures:
[01:24:58] 
[01:24:58] ---- [run-make] run-make-fulldeps/libtest-json stdout ----
[01:24:58] ---- [run-make] run-make-fulldeps/libtest-json stdout ----
[01:24:58] 
[01:24:58] error: make failed
[01:24:58] status: exit code: 2
[01:24:58] command: "make"
[01:24:58] stdout:
[01:24:58] ------------------------------------------
[01:24:58] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/libtest-json'
[01:24:58] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json  --test f.rs
[01:24:58] RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json || true
[01:24:58] cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json | "/usr/bin/python2.7" validate_json.py
[01:24:58] # Compare to output file
[01:24:58] diff output.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json
[01:24:58] 1c1
[01:24:58] < { "type": "suite", "event": "started", "test_count": "4" }
[01:24:58] ---
[01:24:58] > { "type": "suite", "event": "started", "test_count": 4 }
[01:24:58] 10c10
[01:24:58] < { "type": "suite", "event": "failed", "passed": 2, "failed": 1, "allowed_fail": 0, "ignored": 1, "measured": 0, "filtered_out": "0" }
[01:24:58] ---
[01:24:58] > { "type": "suite", "event": "failed", "passed": 2, "failed": 1, "allowed_fail": 0, "ignored": 1, "measured": 0, "filtered_out": 0 }
[01:24:58] Makefile:8: recipe for target 'all' failed
[01:24:58] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/libtest-json'
[01:24:58] ------------------------------------------
[01:24:58] stderr:
[01:24:58] ------------------------------------------
[01:24:58] ------------------------------------------
[01:24:58] make[1]: *** [all] Error 1
[01:24:58] ------------------------------------------
[01:24:58] 
[01:24:58] 
[01:24:58] thread '[run-make] run-make-fulldeps/libtest-json' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:24:58] 
[01:24:58] 
[01:24:58] failures:
[01:24:58]     [run-make] run-make-fulldeps/libtest-json
[01:24:58]     [run-make] run-make-fulldeps/libtest-json
[01:24:58] 
[01:24:58] test result: FAILED. 189 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:24:58] 
[01:24:58] 
[01:24:58] 
[01:24:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodefPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:24:58] 
[01:24:58] 
[01:24:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:58] Build completed unsuccessfully in 0:36:27
[01:24:58] Build completed unsuccessfully in 0:36:27
[01:24:58] make: *** [check] Error 1
[01:24:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2f6a4e72
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:11050c4c:start=1535750674676625774,finish=1535750674751027249,duration=74401475
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09f2804d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07d7bc78
$ dmesg | grep -i kill
