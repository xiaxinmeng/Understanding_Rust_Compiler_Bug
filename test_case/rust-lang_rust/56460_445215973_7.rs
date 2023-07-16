\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-55850.rs","byte_start":1020,"byte_end":1032,"line_start":38,"line_end":38,"column_start":9,"column_end":21,"is_primary":false,"text":[{"text":"        yield &s[..] //~ ERROR `s` does not live long enough [E0597]","highlight_start":9,"highlight_end":21}],"label":"possible yield occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-55850.rs","byte_start":1027,"byte_end":1028,"line_start":38,"line_end":38,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"        yield &s[..] //~ ERROR `s` does not live long enough [E0597]","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0626]: borrow may still be in use when generator yields\n  --> /checkout/src/test/ui/nll/issue-55850.rs:38:16\n   |\nLL |         yield &s[..] //~ ERROR `s` does not live long enough [E0597]\n   |         -------^---- possible yield occurs here\n\n"}
[01:07:48] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:07:48] {"message":"Some errors occurred: E0597, E0626.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0597, E0626.\n"}
[01:07:48] {"message":"For more information about an error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0597`.\n"}
[01:07:48] ------------------------------------------
[01:07:48] 
[01:07:48] thread '[ui (nll)] ui/nll/issue-55850.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[01:07:48] 
---
[01:07:48] 
[01:07:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:07:48] 
[01:07:48] 
[01:07:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:07:48] 
[01:07:48] 
[01:07:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:48] Build completed unsuccessfully in 0:07:27
[01:07:48] Build completed unsuccessfully in 0:07:27
[01:07:48] make: *** [check] Error 1
[01:07:48] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cbe9e59
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 12:16:18 UTC 2018
---
travis_time:end:07e0c38a:start=1544184980615017199,finish=1544184980623643880,duration=8626681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:210d0fa1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b06d66e
travis_time:start:1b06d66e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01c25ad1
$ dmesg | grep -i kill
