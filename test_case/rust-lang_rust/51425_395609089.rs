plain
[00:51:17] ..i..............................................................................i..................
[00:51:23] ....................................................................................................
[00:51:29] ....................................................................................................
[00:51:35] ....................................................................................................
[00:51:40] ..............i.................iiiiiiiii...................................................
[00:51:40] 
[00:51:40] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:52:34] ..i..............................................................................i..................
[00:52:39] ....................................................................................................
[00:52:45] ....................................................................................................
[00:52:51] ....................................................................................................
[00:52:56] ..............i.................iiiiiiiii...................................................
[00:52:56] 
[00:52:56]  finished in 75.487
[00:52:56] travis_fold:end:test_ui_nll

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:42] 
[01:13:42] running 237 tests
[01:15:12] ....................i...................................................F...........................
usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34473" "/checkout/src/test/rustdoc/issue-34473.rs"
[01:16:16] ------------------------------------------
[01:16:16] 
[01:16:16] ------------------------------------------
[01:16:16] stderr:
[01:16:16] stderr:
[01:16:16] ------------------------------------------
[01:16:16] 18: @!has check failed
[01:16:16]  `PATTERN` did not match
[01:16:16]  // @!has - SomeTypeWithLongName
[01:16:16] Encountered 1 errors
[01:16:16] 
[01:16:16] ------------------------------------------
[01:16:16] 
---
[01:16:16] 
[01:16:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:16:16] 
[01:16:16] 
[01:16:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:16] 
[01:16:16] 
[01:16:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:16] Build completed unsuccessfully in 0:27:31
[01:16:16] Build completed unsuccessfully in 0:27:31
[01:16:16] Makefile:58: recipe for target 'check' failed
[01:16:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2eb0b340
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
