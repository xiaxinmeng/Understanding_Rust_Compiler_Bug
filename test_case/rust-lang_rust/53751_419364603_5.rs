\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/self/self_type_keyword-2.rs","byte_start":749,"byte_end":753,"line_start":19,"line_end":19,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        Self => (),","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered --> /checkout/src/test/ui/self/self_type_keyword-2.rs:22:18\n   |\nLL |         Foo { x: Self } => (),\n   |                  ^^^^\n   |\n   = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable\n\n"}
[00:47:41] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:47:41] {"message":"Some errors occurred: E0432, E0531, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0432, E0531, E0658.\n"}
[00:47:41] {"message":"For more information about an error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0432`.\n"}
[00:47:41] ------------------------------------------
[00:47:41] 
[00:47:41] thread '[ui] ui/self/self_type_keyword-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:41] 
[00:47:41] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:47:41] 
[00:47:41] 
[00:47:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:41] 
[00:47:41] 
[00:47:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:41] Build completed unsuccessfully in 0:03:11
[00:47:41] Build completed unsuccessfully in 0:03:11
[00:47:41] Makefile:58: recipe for target 'check' failed
[00:47:41] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0339cbcc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04ef5ee6:start=1536309039199330616,finish=1536309039218963448,duration=19632832
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07e00ab8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12e93ce5
travis_time:start:12e93ce5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:071faeb2
$ dmesg | grep -i kill
