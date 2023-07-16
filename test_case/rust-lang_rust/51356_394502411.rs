plain
[00:41:35] ............................................................................i.......................
[00:41:39] ....................................................................................................
[00:41:45] ....................................................................................................
[00:41:50] ....................................................................................................
[00:41:55] .........i.................iiiiiiiii...................................................
[00:41:55] 
[00:41:55] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:42:42] ............................................................................i.......................
[00:42:47] ....................................................................................................
[00:42:51] ....................................................................................................
[00:42:57] ....................................................................................................
[00:43:01] .........i.................iiiiiiiii...................................................
[00:43:01] 
[00:43:01]  finished in 66.207
[00:43:01] travis_fold:end:test_ui_nll

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:27] 
[00:55:27] running 92 tests
[00:57:10] ............F..............................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[00:59:43] failures:
[00:59:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:59:43] 
[00:59:43] ---- [run-pass] run-pass-fulldeps/issue-11881.rs stdout ----
[00:59:43] ---- [run-pass] run-pass-fulldeps/issue-11881.rs stdout ----
[00:59:43] 
[00:59:43] error: compilation failed!
[00:59:43] status: exit code: 101
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-11881.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-11881/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-11881/auxiliary"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[00:59:43]   --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:25:10
[00:59:43]    |
[00:59:43] 25 | #[derive(Encodable)]
[00:59:43] 
[00:59:43] 
[00:59:43] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[00:59:43]   --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:30:10
[00:59:43]    |
[00:59:43] 30 | #[derive(Encodable)]
[00:59:43] 
[00:59:43] warning: unused import: `std::fmt`
[00:59:43]   --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:18:5
[00:59:43]    |
---
[00:59:43] 
[00:59:43] error[E0308]: mismatched types
[00:59:43]   --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:45:44
[00:59:43]    |
[00:59:43] 45 |     let mut encoder = opaque::Encoder::new(wr);
[00:59:43]    |                                            ^^ expected struct `std::vec::Vec`, found mutable reference
[00:59:43]    = note: expected type `std::vec::Vec<u8>`
[00:59:43]    = note: expected type `std::vec::Vec<u8>`
[00:59:43]               found type `&mut std::io::Cursor<std::vec::Vec<u8>>`
[00:59:43] error: aborting due to previous error
[00:59:43] 
[00:59:43] For more information about this error, try `rustc --explain E0308`.
[00:59:43] 
---
[00:59:43] test result: FAILED. 91 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:43] 
[00:59:43] 
[00:59:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:43] Build completed unsuccessfully in 0:20:13
[00:59:43] Build completed unsuccessfully in 0:20:13
[00:59:43] make: *** [check] Error 1
[00:59:43] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21543c9a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
