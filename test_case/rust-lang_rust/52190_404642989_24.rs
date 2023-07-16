\nstruct FancyNum {\n    num: u8nd":24,"is_primary":true,"text":[{"text":"        *y.pointer += 1;","highlight_start":9,"highlight_end":24}],"label":"assignment to borrowed `*y.pointer` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0506]: cannot assign to `*y.pointer` because it is borrowed (Ast)\n  --> /checkout/src/test/ui/issue-45697-1.rs:30:9\n   |\nLL |         let z = copy_borrowed_ptr(&mut y);\n   |                                        - borrow of `*y.pointer` occurs here\nLL |         *y.pointer += 1;\n   |         ^^^^^^^^^^^^^^^ assignment to borrowed `*y.pointer` occurs here\n\n"}
[00:44:22] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:44:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:22] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:22] {"message":"For more information about this error, try `rustc --explain E0506`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0506`.\n"}
[00:44:22] error: internal compiler error: unexpected panic
[00:44:22] 
---
[00:44:22]     [ui] ui/issue-46472.rs
[00:44:22]     [ui] ui/issue-47184.rs
[00:44:22]     [ui] ui/issue-47646.rs
[00:44:22]     [ui] ui/issue-48132.rs
[00:44:22]     [ui] ui/issue-4817ons " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:22] 
[00:44:22] 
[00:44:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:22] Build completed unsuccessfully in 0:01:20
[00:44:22] Build completed unsuccessfully in 0:01:20
[00:44:22] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11c1e75e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:052b90b4:start=1531427916097782749,finish=1531427916105146062,duration=7363313
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:098d19d8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01dd66ea
$ dmesg | grep -i kill
