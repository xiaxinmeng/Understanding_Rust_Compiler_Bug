plain
[01:11:47] failures:
[01:11:47] 
[01:11:47] ---- [rustdoc] rustdoc/universal-impl-trait.rs stdout ----
[01:11:47] 
[01:11:47] error: htmldocck failed!
[01:11:47] status: exit code: 1
[01:11:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/universal-impl-trait" "/checkout/src/test/rustdoc/universal-impl-trait.rs"
[01:11:47] ------------------------------------------
[01:11:47] 
[01:11:47] ------------------------------------------
[01:11:47] stderr:
[01:11:47] stderr:
[01:11:47] ------------------------------------------
[01:11:47] 46: @matches check failed
[01:11:47]  `PATTERN` did not match
[01:11:47]      // @matches - 'trait\.Read\.html'
[01:11:47] 58: @matches check failed
[01:11:47]  `PATTERN` did not match
[01:11:47]  // @matches - '_: impl .+trait\.Read\.html.+ \+ .+trait\.Clone\.html'
[01:11:47] Encountered 2 errors
[01:11:47] 
[01:11:47] ------------------------------------------
[01:11:47] 
---
[01:11:47] test result: FAILED. 233 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:11:47] 
[01:11:47] 
[01:11:47] 
[01:11:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:47] 
[01:11:47] 
[01:11:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:47] Build completed unsuccessfully in 0:13:29
[01:11:47] Build completed unsuccessfully in 0:13:29
[01:11:47] make: *** [check] Error 1
[01:11:47] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0080291a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
