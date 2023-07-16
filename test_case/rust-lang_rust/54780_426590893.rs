plain
[00:49:13] .................................................................................................... 100/4543
[00:49:16] .................................................................................................... 200/4543
[00:49:19] .................................................................................................... 300/4543
[00:49:22] .................................................................................................... 400/4543
[00:49:25] ..........................................................................F......................... 500/4543
[00:49:34] .................................................................................................... 700/4543
[00:49:39] ................................i...........i....................................................... 800/4543
[00:49:42] ...................................................iiiii............................................ 900/4543
[00:49:45] .................................................................................................... 1000/4543
---
[00:51:29] .................................................................................................... 4100/4543
[00:51:33] ............................i....................................................................... 4200/4543
[00:51:37] .................................................................................................... 4300/4543
[00:51:39] .................................................................................................... 4400/4543
(Self: Foo) :- Implemented(Self: Foo).
[00:51:43] 23    = note: WellFormed(Self: std::marker::Sized) :- Implemented(Self: std::marker::Sized).
[00:51:43] 
[00:51:43] 
[00:51:43] The actual stderr differed from the expected stderr.
[00:51:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1/lower_env1.stderr
[00:51:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1/lower_env1.stderr
[00:51:43] To update references, rerun the tests and pass the `--bless` flag
[00:51:43] To only update this specific test, also pass `--test-args chalkify/lower_env1.rs`
[00:51:43] error: 1 errors occurred comparing output.
[00:51:43] status: exit code: 1
[00:51:43] status: exit code: 1
[00:51:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_env1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1/auxiliary" "-A" "unused"
[00:51:43] ------------------------------------------
[00:51:43] 
[00:51:43] ------------------------------------------
[00:51:43] stderr:
[00:51:43] stderr:
[00:51:43] ------------------------------------------
[00:51:43] {"message":"program clause dump","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/chalkify/lower_env1.rs","byte_start":529,"byte_end":558,"line_start":16,"line_end":16,"column_start":1,"column_end":30,"is_primary":tru"message":"Implemented(Self: Bar) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo) :- FromEnv(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: Foo) :- Implemented(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"WellFormed(Self: std::marker::Sized) :- Implemented(Self: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_env1.rs:19:1\n   |\nLL | #[rustc_dump_env_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).\n   = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).\n   = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).\n   = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).\n   = note: WellFormed(Self: Bar) :- Implemented(Self: Bar), WellFormed(Self: Foo).\n   = note: WellFormed(Self: Foo) :- Implemented(Self: Foo).\n   = note: WellFormed(Self: std::marker::Sized) :- Implemented(Self: std::marker::Sized).\n\n"}
[00:51:43] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:43] ------------------------------------------
[00:51:43] 
[00:51:43] thread '[ui] ui/chalkify/lower_env1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:43] 
[00:51:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:51:43] 
[00:51:43] 
[00:51:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:43] 
[00:51:43] 
[00:51:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:43] Build completed unsuccessfully in 0:03:39
[00:51:43] Build completed unsuccessfully in 0:03:39
[00:51:43] make: *** [check] Error 1
[00:51:43] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01f80d98
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:002d238e:start=1538563544161736853,finish=1538563544171184514,duration=9447661
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c3491a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:030c9774
$ dmesg | grep -i kill
