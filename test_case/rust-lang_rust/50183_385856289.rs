plain
[01:09:25] .............................................................................ii.....................
[01:10:22] .........................................i.............................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:10:32] .......................i.ii..
[01:11:26] ....................................................................................................
[01:11:54] ..iiiiiii...........................................................................................
[01:13:02] ....................................................................................................
[01:13:32] ...........................................................................
[01:13:32] test result: ok. 2956 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[01:13:32] 
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:46] 
[01:26:46] running 231 tests
[01:29:10] ...................i...................................F.....................................FFF.F..
', tools/compiletest/src/runtest.rs:2965:9
[01:31:03] 
[01:31:03] ---- [rustdoc] rustdoc/issue-20727-2.rs stdout ----
[01:31:03]  
[01:31:03]  
[01:31:03] error: htmldocck failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/issue-20727-2.rs"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
[01:31:03] stderr:
[01:31:03] ------------------------------------------
[01:31:03] 30: @has check failed
[01:31:03]  `XPATH PATTERN` did not match
[01:31:03]      // @has - '//*[@class="rust trait"]' 'fn add(self, rhs: RHS) -> Self::Output;'
[01:31:03] Encountered 1 errors
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] 
[01:31:03] thread '[rustdoc] rustdoc/issue-20727-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:31:03] 
[01:31:03] ---- [rustdoc] rustdoc/issue-20727-3.rs stdout ----
[01:31:03]  
[01:31:03] error: htmldocck failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-3.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/issue-20727-3.rs"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
[01:31:03] stderr:
[01:31:03] ------------------------------------------
[01:31:03] 32: @has check failed
[01:31:03]  `XPATH PATTERN` did not match
[01:31:03]      // @has - '//*[@class="rust trait"]' 'fn deref(&self) -> Self::Target;'
[01:31:03] Encountered 1 errors
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] 
[01:31:03] thread '[rustdoc] rustdoc/issue-20727-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:31:03] 
[01:31:03] ---- [rustdoc] rustdoc/issue-20727-4.rs stdout ----
[01:31:03]  
[01:31:03] error: htmldocck failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/issue-20727-4.rs"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
[01:31:03] stderr:
[01:31:03] ------------------------------------------
[01:31:03] 47: @has check failed
[01:31:03]  `XPATH PATTERN` did not match
[01:31:03]      // @has - '//*[@class="rust trait"]' 'fn index_mut(&mut self, index: Idx) -> &mut Self::Output;'
[01:31:03] Encountered 1 errors
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] 
[01:31:03] thread '[rustdoc] rustdoc/issue-20727-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:31:03] 
[01:31:03] ---- [rustdoc] rustdoc/issue-20727.rs stdout ----
[01:31:03]  
[01:31:03] error: htmldocck failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/issue-20727.rs"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
[01:31:03] stderr:
[01:31:03] ------------------------------------------
[01:31:03] 31: @has check failed
[01:31:03]  `XPATH PATTERN` did not match
[01:31:03]      // @has - '//*[@class="rust trait"]' "fn deref(&'a self) -> &'a Self::Target;"
[01:31:03] Encountered 1 errors
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] 
---
[01:31:03] test result: FAILED. 224 passed; 5 failed; 2 ignored; 0 measured; 0 filtered out
[01:31:03] 
[01:31:03] 
[01:31:03] 
[01:31:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:31:03] 
[01:31:03] 
[01:31:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:03] Build completed unsuccessfully in 0:36:01
[01:31:03] Build completed unsuccessfully in 0:36:01
[01:31:03] Makefile:58: recipe for target 'check' failed
[01:31:03] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1067cea5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
