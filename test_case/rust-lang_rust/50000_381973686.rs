plain
Resolving deltas: 100% (616343/616343), completed with 4906 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:43:48] .................................................................................i..................
[00:43:54] ........................i...........................................................................
---
[00:44:35] .....................i...........................................................................i..
[00:44:42] ....................................................................................................
[00:44:48] ...........ii.......................................................................................
[00:44:56] ............................................................................................i.......
---
[00:45:00] thread 'main' panicked at 'malformed condition directive: expected `//[foo]`, found `#[derive(`', tools/compiletest/src/header.rs:437:17
[00:45:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:00]
[00:45:00]
[00:45:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:00] expected success, got: exit code: 101
[00:45:00]
[00:45:00]
[00:45:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:00] Build completed unsuccessfully in 0:02:26
[00:45:00] make: *** [check] Error 1
[00:45:00] Makefile:58: recipe for target 'check' failed
---
$ dmesg | grep -i kill
[   10.544141] init: failsafe main process (1093) killed by TERM signal
