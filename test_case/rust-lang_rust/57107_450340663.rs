plain
[01:22:10] 
[01:22:10] ---- [ui (nll)] ui/thread-local-mutation.rs stdout ----
[01:22:10] diff of stderr:
[01:22:10] 
[01:22:10] - error[E0594]: cannot assign to immutable thread-local static item
[01:22:10] + error[E0594]: cannot assign to immutable static item `S`
[01:22:10] 3    |
[01:22:10] 3    |
[01:22:10] 4 LL |     S = "after"; //~ ERROR cannot assign to immutable thread-local static item
[01:22:10] -    |     ^^^^^^^^^^^
[01:22:10] +    |     ^^^^^^^^^^^ cannot assign
[01:22:10] 6 
[01:22:10] 7 error: aborting due to previous error
[01:22:10] 7 error: aborting due to previous error
[01:22:10] 8 
[01:22:10] 
[01:22:10] 
[01:22:10] The actual stderr differed from the expected stderr.
[01:22:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-mutation.nll/thread-local-mutation.nll.stderr
[01:22:10] To update references, rerun the tests and pass the `--bless` flag
[01:22:10] To only update this specific test, also pass `--test-args thread-local-mutation.rs`
[01:22:10] error: 1 errors occurred comparing output.
[01:22:10] status: exit code: 1
[01:22:10] status: exit code: 1
[01:22:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thread-local-mutation.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-mutation.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-mutation.nll/auxiliary" "-A" "unused"
[01:22:10] ------------------------------------------
[01:22:10] 
[01:22:10] ------------------------------------------
[01:22:10] stderr:
[01:22:10] stderr:
[01:22:10] ------------------------------------------
[01:22:10] {"message":"cannot assign to immutable static item `S`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/thread-local-mutation.rs","byte_start":290,"byte_end":301,"line_start":11,"line_end":11,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    S = \"after\"; //~ ERROR cannot assign to immutable thread-local static item","highlight_start":5,"highlight_end":16}],"label":"cannot assign","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable static item `S`\n  --> /checkout/src/test/ui/thread-local-mutation.rs:11:5\n   |\nLL |     S = \"after\"; //~ ERROR cannot assign to immutable thread-local static item\n   |     ^^^^^^^^^^^ cannot assign\n\n"}
[01:22:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:22:10] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[01:22:10] ------------------------------------------
[01:22:10] 
[01:22:10] thread '[ui (nll)] ui/thread-local-mutation.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:22:10] 
---
[01:22:10] 
[01:22:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:22:10] 
[01:22:10] 
[01:22:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:22:10] 
[01:22:10] 
[01:22:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:10] Build completed unsuccessfully in 0:07:20
[01:22:10] Build completed unsuccessfully in 0:07:20
[01:22:10] make: *** [check] Error 1
[01:22:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:170fff84
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 28 10:53:38 UTC 2018
---
travis_time:end:08a2f860:start=1545994420344206932,finish=1545994420351602241,duration=7395309
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0621f912
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a222176
travis_time:start:0a222176
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14539e57
$ dmesg | grep -i kill
