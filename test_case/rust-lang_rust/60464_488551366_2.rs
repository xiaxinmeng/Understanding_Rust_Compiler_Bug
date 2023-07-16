
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/private-item-doc-test.rs:1:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(private_doc_tests)]
[01:42:22] - 
[01:42:22] - error: aborting due to previous error
[01:42:22] - error: aborting due to previous error
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 19 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-item-doc-test/private-item-doc-test.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args private-item-doc-test.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/private-item-doc-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-item-doc-test" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-item-doc-test/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
---
[01:42:22] test result: FAILED. 0 passed; 25 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:22] 
[01:42:22] 
[01:42:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:22] Build completed unsuccessfully in 0:35:41
[01:42:22] Build completed unsuccessfully in 0:35:41
[01:42:22] make: *** [check] Error 1
[01:42:22] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0787c3d7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 04:40:53 UTC 2019
---
travis_time:end:0749418c:start=1556772055532285936,finish=1556772055537546415,duration=5260479
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09c5270e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a058c30
travis_time:start:1a058c30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a02b60f
$ dmesg | grep -i kill
