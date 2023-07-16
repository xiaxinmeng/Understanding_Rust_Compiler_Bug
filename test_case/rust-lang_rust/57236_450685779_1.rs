\n\nStatics are shared everywhere, and if they refer to mutable data one might\nviolate memory safety since holding multiple mutable references to shared data\nis not allowed.\n\nIf you really want global mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs","byte_start":110,"byte_end":136,"line_start":5,"line_end":5,"column_start":46,"column_end":72,"is_primary":true,"text":[{"text":"pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };","highlight_start":46,"highlight_end":72}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs:5:46\n   |\nLL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };\n   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values\n\n"}
[00:49:12] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:12] {"message":"For more information about this error, try `rustc --explain E0017`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0017`.\n"}
[00:49:12] ------------------------------------------
[00:49:12] 
[00:49:12] thread '[ui] ui/consts/static_mut_containing_mut_ref2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:49:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:12] 
[00:49:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:49:12] 
[00:49:12] 
[00:49:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:12] 
[00:49:12] 
[00:49:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:49:12] Build completed unsuccessfully in 0:44:13
---
travis_time:end:14edcd18:start=1546286496720094691,finish=1546286496729657123,duration=9562432
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0065f4b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:171b4fa0
travis_time:start:171b4fa0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3134d931
$ dmesg | grep -i kill
