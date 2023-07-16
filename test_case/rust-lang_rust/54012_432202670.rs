plain
[00:16:30]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:16:30]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:16:54]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:16:54]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:16:54]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:18:28]    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
[00:18:29]     Finished release [optimized] target(s) in 13m 41s
[00:18:29] Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:18:29] travis_fold:end:stage0-rustc
---
[00:31:07]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:31:07]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:31:07]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:31:07]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:31:25]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:32:16]    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
[00:32:16]     Finished release [optimized] target(s) in 11m 36s
[00:32:16] Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:32:16] travis_fold:end:stage1-rustc
---
[00:47:46] .................................................................................................... 2200/4930
[00:47:53] .................................i.................................................................. 2300/4930
[00:47:59] .................................................................................................... 2400/4930
[00:48:06] .................................................................................................... 2500/4930
[00:48:11] ................................................iiiiiiiii........................................... 2600/4930
[00:48:21] .................................................................................................... 2800/4930
[00:48:26] .................................................................................................... 2900/4930
[00:48:31] ..............................................................................i..................... 3000/4930
[00:48:35] .................................................................................................... 3100/4930
---
[00:55:31] .................................................................................................... 2100/2868
[00:55:41] ..................................................................................................ii 2200/2868
[00:56:04] .....................................................................i....i......................... 2300/2868
[00:56:22] ............i....................................................................................... 2400/2868
[00:56:42] ........................................................................................F........... 2500/2868
[00:57:19] .................................................................................................... 2700/2868
[00:57:31] .................................................................................................... 2800/2868
[00:57:43] ....................................................................
[00:57:43] failures:
---
[00:57:43] 
[00:57:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:57:43] 
[00:57:43] 
[00:57:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:43] 
[00:57:43] 
[00:57:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:43] Build completed unsuccessfully in 0:16:50
[00:57:43] Build completed unsuccessfully in 0:16:50
[00:57:43] make: *** [check] Error 1
[00:57:43] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08fa5d48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:055d69de:start=1540292848691737216,finish=1540292848698017412,duration=6280196
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:277cf28d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cf193c8
travis_time:start:0cf193c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d0c9b80
$ dmesg | grep -i kill
