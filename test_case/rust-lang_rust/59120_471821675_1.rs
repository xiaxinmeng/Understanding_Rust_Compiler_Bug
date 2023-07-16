\n\nPlease be sure that a file corresponding to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-56411.rs","byte_start":303,"byte_end":318,"line_start":13,"line_end":13,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"import!(issue_56411_aux);","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"name the file either issue_56411_aux.rs or issue_56411_aux/mod.rs inside the directory \"/checkout/src/test/ui/issues\"","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0583]: file not found for module `issue_56411_aux`\n  --> /checkout/src/test/ui/issues/issue-56411.rs:13:9\n   |\nLL | import!(issue_56411_aux);\n   |         ^^^^^^^^^^^^^^^\n   |\n   = help: name the file either issue_56411_aux.rs or issue_56411_aux/mod.rs inside the directory \"/checkout/src/test/ui/issues\"\n\n"}
[01:21:41] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[01:21:41] 
[01:21:41] ------------------------------------------
[01:21:41] 
[01:21:41] 
[01:21:41] thread '[ui] ui/issues/issue-56411.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:21:41] 
[01:21:41] ---- [ui] ui/proc-macro/issue-38586.rs stdout ----
[01:21:41] 
[01:21:41] error: aux-build `/checkout/src/test/ui/proc-macro/auxiliary/issue_38586.rs` source not found
[01:21:41] thread '[ui] ui/proc-macro/issue-38586.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:21:41] ---- [ui] ui/proc-macro/issue-50493.rs stdout ----
[01:21:41] 
[01:21:41] 
[01:21:41] error: aux-build `/checkout/src/test/ui/proc-macro/auxiliary/issue_50493.rs` source not found
[01:21:41] thread '[ui] ui/proc-macro/issue-50493.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:21:41] ---- [ui] ui/resolve/issue-19452.rs stdout ----
[01:21:41] 
[01:21:41] 
[01:21:41] error: aux-build `/checkout/src/test/ui/resolve/auxiliary/issue_19452_aux.rs` source not found
[01:21:41] thread '[ui] ui/resolve/issue-19452.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:21:41] ---- [ui] ui/resolve/issue-3907-2.rs stdout ----
[01:21:41] 
[01:21:41] 
[01:21:41] error: aux-build `/checkout/src/test/ui/resolve/auxiliary/issue_3907.rs` source not found
[01:21:41] thread '[ui] ui/resolve/issue-3907-2.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:21:41] ---- [ui] ui/resolve/issue-3907.rs stdout ----
[01:21:41] 
[01:21:41] 
[01:21:41] error: aux-build `/checkout/src/test/ui/resolve/auxiliary/issue_3907.rs` source not found
[01:21:41] thread '[ui] ui/resolve/issue-3907.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:21:41] 
[01:21:41] failures:
[01:21:41]     [ui] ui/e0119/issue-23563.rs
[01:21:41]     [ui] ui/issues/issue-18986.rs
---
[01:21:41] 
[01:21:41] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:21:41] 
[01:21:41] 
[01:21:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:41] 
[01:21:41] 
[01:21:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:41] Build completed unsuccessfully in 0:04:56
[01:21:41] Build completed unsuccessfully in 0:04:56
[01:21:41] make: *** [check] Error 1
[01:21:41] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03af65ba
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 02:13:00 UTC 2019
---
travis_time:end:0386fc62:start=1552356782330011173,finish=1552356782335126260,duration=5115087
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:151c2720
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0119ab8c
travis_time:start:0119ab8c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:053c35ca
$ dmesg | grep -i kill
