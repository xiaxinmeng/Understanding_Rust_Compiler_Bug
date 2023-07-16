\n\nPlease be sure that a file corresponding to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-38190.rs","byte_start":139,"byte_end":150,"line_start":9,"line_end":9,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    m!([mod issue_38190;]);","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"name the file either issue_38190.rs or issue_38190/mod.rs inside the directory \"/checkout/src/test/run-pass/issues/auxiliary\"","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0583]: file not found for module `issue_38190`\n  --> /checkout/src/test/run-pass/issues/issue-38190.rs:9:13\n   |\nLL |     m!([mod issue_38190;]);\n   |             ^^^^^^^^^^^\n   |\n   = help: name the file either issue_38190.rs or issue_38190/mod.rs inside the directory \"/checkout/src/test/run-pass/issues/auxiliary\"\n\n"}
[01:16:07] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[01:16:07] 
[01:16:07] ------------------------------------------
[01:16:07] 
[01:16:07] 
[01:16:07] thread '[run-pass] run-pass/issues/issue-38190.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:16:07] 
[01:16:07] ---- [run-pass] run-pass/issues/issue-40469.rs stdout ----
[01:16:07] 
[01:16:07] error: test compilation failed although it shouldn't!
[01:16:07] status: exit code: 1
[01:16:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-40469.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-40469/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-40469/auxiliary"
[01:16:07] ------------------------------------------
[01:16:07] 
[01:16:07] ------------------------------------------
[01:16:07] stderr:
[01:16:07] stderr:
[01:16:07] ------------------------------------------
[01:16:07] {"message":"couldn't read /checkout/src/test/run-pass/issues/auxiliary/issue_40469.rs: No such file or directory (os error 2)","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-40469.rs","byte_start":65,"byte_end":102,"line_start":6,"line_end":6,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"include!(\"auxiliary/issue_40469.rs\");","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: couldn't read /checkout/src/test/run-pass/issues/auxiliary/issue_40469.rs: No such file or directory (os error 2)\n  --> /checkout/src/test/run-pass/issues/issue-40469.rs:6:1\n   |\nLL | include!(\"auxiliary/issue_40469.rs\");\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:16:07] 
[01:16:07] ------------------------------------------
[01:16:07] 
[01:16:07] thread '[run-pass] run-pass/issues/issue-40469.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:16:07] thread '[run-pass] run-pass/issues/issue-40469.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:16:07] 
[01:16:07] ---- [run-pass] run-pass/issues/issue24687-embed-debuginfo/main.rs stdout ----
[01:16:07] 
[01:16:07] error: aux-build `/checkout/src/test/run-pass/issues/issue24687-embed-debuginfo/auxiliary/issue24687-lib.rs` source not found
[01:16:07] thread '[run-pass] run-pass/issues/issue24687-embed-debuginfo/main.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2043:9
[01:16:07] 
[01:16:07] failures:
[01:16:07]     [run-pass] run-pass/issues/issue-26873-multifile.rs
[01:16:07]     [run-pass] run-pass/issues/issue-3136-b.rs
---
[01:16:07] 
[01:16:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:16:07] 
[01:16:07] 
[01:16:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:07] 
[01:16:07] 
[01:16:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:07] Build completed unsuccessfully in 0:10:31
[01:16:07] Build completed unsuccessfully in 0:10:31
[01:16:07] Makefile:48: recipe for target 'check' failed
[01:16:07] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bf00a30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 02:43:07 UTC 2019
---
travis_time:end:14e5b4fa:start=1552444989124012627,finish=1552444989179625811,duration=55613184
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18cb2aac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:28cabb5f
$ dmesg | grep -i kill
