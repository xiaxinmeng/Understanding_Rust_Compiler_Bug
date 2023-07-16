\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/generator-with-nll.rs","byte_start":914,"byte_end":922,"line_start":22,"line_end":22,"column_start":9,"column_end":17,"is_primary":false,"text":[{"text":"        yield ();","highlight_start":9,"highlight_end":17}],"label":"possible yield occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/generator-with-nll.rs","byte_start":767,"byte_end":771,"line_start":20,"line_end":20,"column_start":22,"column_end":26,"is_primary":true,"text":[{"text":"        let b = &mut true; //~ ERROR borrow may still be in use when generator yields (Ast)","highlight_start":22,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0626]: borrow may still be in use when generator yields (Ast)\n  --> /checkout/src/test/ui/generator/generator-with-nll.rs:20:22\n   |\nLL |         let b = &mut true; //~ ERROR borrow may still be in use when generator yields (Ast)\n   |                      ^^^^\nLL |         //~^ borrow may still be in use when generator yields (Mir)\nLL |         yield ();\n   |         -------- possible yield occurs here\n\n"}
[00:46:44] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:46:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:44] test result: FAILED. 1470 passed; 86 failed; 5 ignored; 0 measured; 0 filtered out
[00:46:44] 
[00:46:44] 
[00:46:44] 
[00:46:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:44] 
[00:46:44] 
[00:46:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:44] Build completed unsuccessfully in 0:01:26
[00:46:44] Build completed unsuccessfully in 0:01:26
[00:46:44] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0536706a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00bbad1e:start=1531504794718181845,finish=1531504794724908651,duration=6726806
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03aa7712
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15a86dd4
$ dmesg | grep -i kill
