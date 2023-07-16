plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:43] 
[01:00:43] running 241 tests
[01:01:56] ..................F.i...............................................................................
[01:02:39] ..............i...................................................................................F.
[01:02:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:02:50] failures:
[01:02:50] 
[01:02:50] ---- [rustdoc] rustdoc/cross-crate-links.rs stdout ----
[01:02:50] ---- [rustdoc] rustdoc/cross-crate-links.rs stdout ----
[01:02:50] 
[01:02:50] error: htmldocck failed!
[01:02:50] status: exit code: 1
[01:02:50] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-links" "/checkout/src/test/rustdoc/cross-crate-links.rs"
[01:02:50] ------------------------------------------
[01:02:50] 
[01:02:50] ------------------------------------------
[01:02:50] stderr:
[01:02:50] stderr:
[01:02:50] ------------------------------------------
[01:02:50] 69: @has check failed
[01:02:50]  `XPATH PATTERN` did not match
[01:02:50]  // @has 'foo/index.html' '//a[@href="../foo/macro.foo_macro.html"]' 'foo_macro'
[01:02:50] Encountered 1 errors
[01:02:50] 
[01:02:50] ------------------------------------------
[01:02:50] 
[01:02:50] 
[01:02:50] thread '[rustdoc] rustdoc/cross-crate-links.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:02:50] note: R/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:50] 
[01:02:50] 
[01:02:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:50] Build completed unsuccessfully in 0:22:33
[01:02:50] Build completed unsuccessfully in 0:22:33
[01:02:50] Makefile:58: recipe for target 'check' failed
[01:02:50] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:399a0e12
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
