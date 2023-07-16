plain
[00:50:37] .................................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:50:38] ...
[00:50:51] ....................................................................................................
[00:51:29] ii..............................................................i...................................
[00:51:55] .................i.ii..........................................................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:52:21] .........................iiiiiii....................................................................
[00:52:40] ....................................................................................................
[00:52:52] ....................................................................................................
[00:53:08] ................................................................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:45] 
[00:56:45] running 90 tests
[00:58:26] ..F........................................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:01:00] failures:
[01:01:00] 
[01:01:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:01:00] ---- [run-pass] run-pass-fulldeps/compiler-calls.rs stdout ----
[01:01:00] ---- [run-pass] run-pass-fulldeps/compiler-calls.rs stdout ----
[01:01:00] 
[01:01:00] error: compilation failed!
[01:01:00] status: exit code: 101
[01:01:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/compiler-calls.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/auxiliary"
[01:01:00] ------------------------------------------
[01:01:00] 
[01:01:00] ------------------------------------------
[01:01:00] stderr:
[01:01:00] stderr:
[01:01:00] ------------------------------------------
[01:01:00] error[E0053]: method `build_controller` has an incompatible type for trait
[01:01:00]    |
[01:01:00]    |
[01:01:00] 80 | /     fn build_controller(&mut self,
[01:01:00] 81 | |                         _: &Session,
[01:01:00] 82 | |                         _: &getopts::Matches)
[01:01:00] 83 | |                         -> driver::CompileController<'a> {
[01:01:00] 84 | |         panic!("This shouldn't be called");
[01:01:00] 85 | |     }
[01:01:00]    | |_____^ expected struct `std::boxed::Box`, found mutable reference
[01:01:00]    |
[01:01:00]    = note: expected type `fn(std::boxed::Box<TestCalls>, &rustc::session::Session, &getopts::Matches) -> rustc_driver::driver::CompileController<'a>`
[01:01:00]               found type `fn(&mut TestCalls, &rustc::session::Session, &getopts::Matches) -> rustc_driver::driver::CompileController<'a>`
[01:01:00] error: aborting due to previous error
[01:01:00] 
[01:01:00] For more information about this error, try `rustc --explain E0053`.
[01:01:00] 
---
[01:01:00] test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:01:00] 
[01:01:00] 
[01:01:00] 
[01:01:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:00] 
[01:01:00] 
[01:01:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:00] Build completed unsuccessfully in 0:19:05
[01:01:00] Build completed unsuccessfully in 0:19:05
[01:01:00] make: *** [check] Error 1
[01:01:00] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a44bb32
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
