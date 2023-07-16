\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-59029-1.rs","byte_start":101,"byte_end":110,"line_start":5,"line_end":5,"column_start":46,"column_end":55,"is_primary":true,"text":[{"text":"trait MkSvc<Target, Req> = Svc<Target> where Self::Res: Svc<Req>;","highlight_start":46,"highlight_end":55}],"label":"associated type `Res` not found","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0220]: associated type `Res` not found for `Self`\n  --> /checkout/src/test/ui/issues/issue-59029-1.rs:5:46\n   |\nLL | trait MkSvc<Target, Req> = Svc<Target> where Self::Res: Svc<Req>;\n   |                                              ^^^^^^^^^ associated type `Res` not found\n\n"}
[01:10:59] {"message":"For more information about this error, try `rustc --explain E0220`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0220`.\n"}
[01:10:59] 
[01:10:59] ------------------------------------------
[01:10:59] 
---
[01:10:59] 
[01:10:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:10:59] 
[01:10:59] 
[01:10:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:59] 
[01:10:59] 
[01:10:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:59] Build completed unsuccessfully in 0:04:15
[01:10:59] Build completed unsuccessfully in 0:04:15
[01:10:59] make: *** [check] Error 1
[01:10:59] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01340268
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 20:31:00 UTC 2019
---
travis_time:end:00938d7c:start=1552422661837442010,finish=1552422661842126205,duration=4684195
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b294f20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02cd9274
travis_time:start:02cd9274
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f66ae0
$ dmesg | grep -i kill
