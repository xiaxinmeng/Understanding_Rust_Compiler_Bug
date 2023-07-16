\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs","byte_start":1047,"byte_end":1072,"line_start":28,"line_end":28,"column_start":10,"column_end":35,"is_primary":true,"text":[{"text":"#[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope","highlight_start":10,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(use_extern_macros)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: non-ident macro paths are experimental (see issue #35896)\n  --> /checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs:28:10\n   |\nLL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope\n   |          ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(use_extern_macros)] to the crate attributes to enable\n\n"}
[00:52:35] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:52:35] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:52:35] ------------------------------------------
[00:52:35] 
[00:52:35] thread '[ui] ui-fulldeps/proc-macro/generate-mod.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:52:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:52:35] test result: FAILED. 19 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:52:35] 
[00:52:35] 
[00:52:35] 
[00:52:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:35] 
[00:52:35] 
[00:52:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:35] Build completed unsuccessfully in 0:10:10
[00:52:35] Build completed unsuccessfully in 0:10:10
[00:52:35] Makefile:58: recipe for target 'check' failed
[00:52:35] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2fb47c2a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:021fdfd4:start=1531358323247796390,finish=1531358323255230877,duration=7434487
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e19bd20
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0608a543
$ dmesg | grep -i kill
