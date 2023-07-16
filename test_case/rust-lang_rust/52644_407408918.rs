plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:05] 
[00:52:05] running 96 tests
[00:53:52] ..F...F................................................F...........test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[00:55:14] failures:
[00:55:14] 
[00:55:14] ---- [run-pass] run-pass-fulldeps/compiler-calls.rs stdout ----
[00:55:14] 
[00:55:14] 
[00:55:14] error: compilation failed!
[00:55:14] status: exit code: 1
[00:55:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/compiler-calls.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/compiler-calls/auxiliary"
[00:55:14] ------------------------------------------
[00:55:14] 
[00:55:14] ------------------------------------------
[00:55:14] stderr:
[00:55:14] stderr:
[00:55:14] ------------------------------------------
[00:55:14] warning: the feature `core` has been stable since 1.6.0 and no longer requires an attribute to enable
[00:55:14]    |
[00:55:14] 16 | #![feature(core)]
[00:55:14]    |            ^^^^
[00:55:14]    |
---
[00:55:14] ---- [run-pass] run-pass-fulldeps/deriving-global.rs stdout ----
[00:55:14] 
[00:55:14] error: compilation failed!
[00:55:14] status: exit code: 1
[00:55:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/deriving-global.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-global/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-global/auxiliary"
[00:55:14] ------------------------------------------
[00:55:14] 
[00:55:14] ------------------------------------------
[00:55:14] stderr:
[00:55:14] stderr:
[00:55:14] ------------------------------------------
[00:55:14] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[00:55:14]    |
[00:55:14] 23 |                Encodable, Decodable)]
[00:55:14]    |                           ^^^^^^^^^
[00:55:14] 
[00:55:14] 
[00:55:14] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[00:55:14]    |
[00:55:14] 23 |                Encodable, Decodable)]
[00:55:14]    |                ^^^^^^^^^
[00:55:14] 
[00:55:14] 
[00:55:14] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[00:55:14]    |
[00:55:14] 30 |                Encodable, Decodable)]
[00:55:14]    |                           ^^^^^^^^^
[00:55:14] 
[00:55:14] 
[00:55:14] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[00:55:14]    |
[00:55:14] 30 |                Encodable, Decodable)]
[00:55:14]    |                ^^^^^^^^^
[00:55:14] 
[00:55:14] 
[00:55:14] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[00:55:14]    |
[00:55:14] 37 |                Encodable, Decodable)]
[00:55:14]    |                           ^^^^^^^^^
[00:55:14] 
[00:55:14] 
[00:55:14] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[00:55:14]    |
[00:55:14] 37 |                Encodable, Decodable)]
[00:55:14]    |                ^^^^^^^^^
[00:55:14] 
[00:55:14] 
[00:55:14] error[E0635]: unknown feature `rand`
[00:55:14]   --> /checkout/src/test/run-pass-fulldeps/deriving-global.rs:11:12
[00:55:14]    |
[00:55:14] 11 | #![feature(rand, rustc_private)]
[00:55:14] 
[00:55:14] error: aborting due to previous error
[00:55:14] 
[00:55:14] For more information about this error, try `rustc --explain E0635`.
---
[00:55:14] ---- [run-pass] run-pass-fulldeps/proc-macro/attr-stmt-expr.rs stdout ----
[00:55:14] 
[00:55:14] error: compilation failed!
[00:55:14] status: exit code: 1
[00:55:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/attr-stmt-expr.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/attr-stmt-expr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/attr-stmt-expr/auxiliary"
[00:55:14] ------------------------------------------
[00:55:14] 
[00:55:14] ------------------------------------------
[00:55:14] stderr:
[00:55:14] stderr:
[00:55:14] ------------------------------------------
[00:55:14] error[E0635]: unknown feature `proc_macro_stmt`
[00:55:14]    |
[00:55:14]    |
[00:55:14] 14 | #![feature(use_extern_macros, stmt_expr_attributes, proc_macro_stmt, proc_macro_expr)]
[00:55:14] 
[00:55:14] error: aborting due to previous error
[00:55:14] 
[00:55:14] For more information about this error, try `rustc --explain E0635`.
---
[00:55:14] 
[00:55:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:14] 
[00:55:14] 
[00:55:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:14] 
[00:55:14] 
[00:55:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:14] Build completed unsuccessfully in 0:13:49
[00:55:14] Build completed unsuccessfully in 0:13:49
[00:55:14] Makefile:58: recipe for target 'check' failed
[00:55:14] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11129c78
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08d04a60:start=1532439361674039379,finish=1532439361685828850,duration=11789471
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14df96ca
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02a9a87e
travis_time:start:02a9a87e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0179c970
$ dmesg | grep -i kill
