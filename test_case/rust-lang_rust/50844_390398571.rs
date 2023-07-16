plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:40] 
[01:20:40] running 90 tests
[01:22:37] ..F.......................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:27:46] failures:
[01:27:46] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:27:46] 
[01:27:46] ---- [run-pass] run-pass-fulldeps/compiler-calls.rs stdout ----
[01:27:46] ---- [run-pass] run-pass-fulldeps/compiler-calls.rs stdout ----
[01:27:46] 
[01:27:46] error: compilation failed!
[01:27:46] status: exit code: 101
[01:27:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/compiler-calls.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/auxiliary"
[01:27:46] ------------------------------------------
[01:27:46] 
[01:27:46] ------------------------------------------
[01:27:46] stderr:
[01:27:46] stderr:
[01:27:46] ------------------------------------------
[01:27:46] error[E0597]: `count` does not live long enough
[01:27:46]    |
[01:27:46]    |
[01:27:46] 92 |         let tc = TestCalls { count: &mut count };
[01:27:46]  rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:27:46] 
[01:27:46] 
[01:27:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:46] Build completed unsuccessfully in 0:28:13
[01:27:46] Build completed unsuccessfully in 0:28:13
[01:27:46] make: *** [check] Error 1
[01:27:46] Makefile:58: recipe for target 'check' failed
