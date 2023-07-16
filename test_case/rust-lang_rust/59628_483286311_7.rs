\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs","byte_start":64,"byte_end":75,"line_start":4,"line_end":4,"column_start":27,"column_end":38,"is_primary":true,"text":[{"text":"    let _result = &Ok(42).as_deref_ok();","highlight_start":27,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the method `as_deref_ok` exists but the following trait bounds were not satisfied:\n`{integer} : std::ops::Deref`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"there is a method with a similar name","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs","byte_start":64,"byte_end":75,"line_start":4,"line_end":4,"column_start":27,"column_end":38,"is_primary":true,"text":[{"text":"    let _result = &Ok(42).as_deref_ok();","highlight_start":27,"highlight_end":38}],"label":null,"suggested_replacement":"as_deref_err","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `as_deref_ok` found for type `std::result::Result<{integer}, _>` in the current scope\n  --> /checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs:4:27\n   |\nLL |     let _result = &Ok(42).as_deref_ok();\n   |                           ^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_err`\n   |\n   = note: the method `as_deref_ok` exists but the following trait bounds were not satisfied:\n           `{integer} : std::ops::Deref`\n\n"}
[01:08:43] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
[01:08:43] thread '[ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3469:9
[01:08:43] 
[01:08:43] 
[01:08:43] failures:
[01:08:43]     [ui] ui/issues/issue-50264-inner-deref-trait/option-as_deref.rs
[01:08:43]     [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref.rs
[01:08:43]     [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_err.rs
[01:08:43]     [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut.rs
[01:08:43]     [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_err.rs
[01:08:43]     [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut_ok.rs
[01:08:43]     [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_ok.rs
[01:08:43] test result: FAILED. 5515 passed; 7 failed; 21 ignored; 0 measured; 0 filtered out
[01:08:43] 
[01:08:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:08:43] 
[01:08:43] 
[01:08:43] 
[01:08:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:43] 
[01:08:43] 
[01:08:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:43] Build completed unsuccessfully in 0:04:24
[01:08:43] Build completed unsuccessfully in 0:04:24
[01:08:43] Makefile:48: recipe for target 'check' failed
[01:08:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00e3d19f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 15 14:55:14 UTC 2019
---
travis_time:end:00231b85:start=1555340115897262991,finish=1555340115902042764,duration=4779773
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07cb7f86
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:114c6e8e
travis_time:start:114c6e8e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0279b23b
$ dmesg | grep -i kill
