plain
[00:17:38]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:17:38]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:17:38]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:18:07]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:18:07]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:19:35]     Finished release [optimized] target(s) in 14m 35s
[00:19:35] Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:19:35] travis_fold:end:stage0-rustc

---
[00:32:23]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:32:35]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:32:35]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:33:01]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:33:01]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:34:00]    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
[00:34:00]     Finished release [optimized] target(s) in 12m 07s
[00:34:00] Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:34:00] travis_fold:end:stage1-rustc
---
[00:50:11] .................................................................................................... 2200/4940
[00:50:18] .................................i.................................................................. 2300/4940
[00:50:24] .................................................................................................... 2400/4940
[00:50:32] .................................................................................................... 2500/4940
[00:50:37] ................................................iiiiiiiii........................................... 2600/4940
[00:50:43] ..................................................................................................ii 2700/4940
[00:50:52] .................................................................................................... 2900/4940
[00:50:57] ........................................................................................i........... 3000/4940
[00:51:01] .................................................................................................... 3100/4940
[00:51:06] ...............................................i.i..ii.............................................. 3200/4940
---
[00:58:13] .................................................................................................... 2100/2868
[00:58:24] ..................................................................................................ii 2200/2868
[00:58:49] .....................................................................i....i......................... 2300/2868
[00:59:08] ............i....................................................................................... 2400/2868
[00:59:29] ..........................................................................................F......... 2500/2868
[01:00:09] .................................................................................................... 2700/2868
[01:00:22] .................................................................................................... 2800/2868
[01:00:35] ....................................................................
[01:00:35] failures:
---
[01:00:35] 
[01:00:35] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:00:35] 
[01:00:35] 
[01:00:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:35] 
[01:00:35] 
[01:00:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:35] Build completed unsuccessfully in 0:17:25
[01:00:35] Build completed unsuccessfully in 0:17:25
[01:00:35] make: *** [check] Error 1
[01:00:35] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0579d59a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:09cc08a4:start=1540306545989055614,finish=1540306545992791409,duration=3735795
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03e21c07
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21aff3c1
travis_time:start:21aff3c1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:272c7fa8
$ dmesg | grep -i kill
