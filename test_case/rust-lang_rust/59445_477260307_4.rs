\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0225.rs","byte_start":54,"byte_end":68,"line_start":3,"line_end":3,"column_start":29,"column_end":43,"is_primary":false,"text":[{"text":"trait Foo = std::io::Read + std::io::Write;","highlight_start":29,"highlight_end":43}],"label":"non-auto additional trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0225.rs","byte_start":237,"byte_end":240,"line_start":8,"line_end":8,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"    let _: Box<Foo>;","highlight_start":16,"highlight_end":19}],"label":"expanded from this trait alias","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0225]: only auto traits can be used as additional traits in a trait object\n  --> /checkout/src/test/ui/error-codes/E0225.rs:8:16\n   |\nLL | trait Foo = std::io::Read + std::io::Write;\n   |                             -------------- non-auto additional trait\n...\nLL |     let _: Box<Foo>;\n   |                ^^^ expanded from this trait alias\n\n"}
[01:15:25] {"message":"For more information about this error, try `rustc --explain E0225`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0225`.\n"}
[01:15:25] 
[01:15:25] ------------------------------------------
[01:15:25] 
---
[01:15:25] 
[01:15:25] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:15:25] 
[01:15:25] 
[01:15:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:25] 
[01:15:25] 
[01:15:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:25] Build completed unsuccessfully in 0:04:24
[01:15:25] Build completed unsuccessfully in 0:04:24
[01:15:25] Makefile:48: recipe for target 'check' failed
[01:15:25] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1386d7bc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 27 17:04:53 UTC 2019
---
travis_time:end:089ff304:start=1553706294481004769,finish=1553706294486440948,duration=5436179
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09ec725c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:036c9426
travis_time:start:036c9426
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b60872a
$ dmesg | grep -i kill
