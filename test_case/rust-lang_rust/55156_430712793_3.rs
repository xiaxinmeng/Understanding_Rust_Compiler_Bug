\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-52717.rs","byte_start":566,"byte_end":569,"line_start":19,"line_end":19,"column_start":12,"column_end":15,"is_primary":true,"text":[{"text":"    A::A { fob } => { println!(\"{}\", fob); }","highlight_start":12,"highlight_end":15}],"label":"variant `A::A` does not have this field","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issue-52717.rs","byte_start":566,"byte_end":569,"line_start":19,"line_end":19,"column_start":12,"column_end":15,"is_primary":true,"text":[{"text":"    A::A { fob } => { println!(\"{}\", fob); }","highlight_start":12,"highlight_end":15}],"label":null,"suggested_replacement":"foo","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0026]: variant `A::A` does not have a field named `fob`\n  --> /checkout/src/test/ui/issue-52717.rs:19:12\n   |\nLL |     A::A { fob } => { println!(\"{}\", fob); }\n   |            ^^^\n   |            |\n   |            variant `A::A` does not have this field\n   |            help: did you mean: `foo`\n\n"}
[00:45:50] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:50] {"message":"For more information about this error, try `rustc --explain E0026`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0026`.\n"}
[00:45:50] ------------------------------------------
[00:45:50] 
[00:45:50] thread '[ui] ui/issue-52717.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:45:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:50] 
[00:45:50] ---- [ui] ui/issues/issue-17800.rs stdout ----
[00:45:50] 
[00:45:50] error: /checkout/src/test/ui/issues/issue-17800.rs:18: e-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:50] 
[00:45:50] 
[00:45:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:50] Build completed unsuccessfully in 0:03:22
[00:45:50] Build completed unsuccessfully in 0:03:22
[00:45:50] Makefile:58: recipe for target 'check' failed
[00:45:50] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:009e5c3c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:090c66f7:start=1539796496655428196,finish=1539796496662459304,duration=7031108
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0186b2fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02fa484e
$ dmesg | grep -i kill
