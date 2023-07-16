\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-ptr-nonnull.rs","byte_start":263,"byte_end":280,"line_start":9,"line_end":9,"column_start":37,"column_end":54,"is_primary":true,"text":[{"text":"    let x: &'static NonNull<u32> = &(non_null.cast());","highlight_start":37,"highlight_end":54}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-nonnull.rs","byte_start":338,"byte_end":339,"line_start":11,"line_end":11,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-nonnull.rs","byte_start":238,"byte_end":259,"line_start":9,"line_end":9,"column_start":12,"column_end":33,"is_primary":false,"text":[{"text":"    let x: &'static NonNull<u32> = &(non_null.cast());","highlight_start":12,"highlight_end":33}],"label":"type annotation requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-ptr-nonnull.rs:9:37\n   |\nLL |     let x: &'static NonNull<u32> = &(non_null.cast());\n   |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\n   |            |\n   |            type annotation requires that borrow lasts for `'static`\nLL |     //~^ ERROR borrowed value does not live long enough\nLL | }\n   | - temporary value is freed at the end of this statement\n\n"}
[01:38:49] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:38:49] 
[01:38:49] ------------------------------------------
[01:38:49] 
---
[01:38:49] 
[01:38:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:38:49] 
[01:38:49] 
[01:38:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:38:49] 
[01:38:49] 
[01:38:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:49] Build completed unsuccessfully in 0:08:54
[01:38:49] Build completed unsuccessfully in 0:08:54
[01:38:49] Makefile:48: recipe for target 'check' failed
[01:38:49] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02f199f6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 27 07:54:53 UTC 2019
---
travis_time:end:3a53450c:start=1551254096049891545,finish=1551254096061404110,duration=11512565
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0339bc84
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26cd498c
travis_time:start:26cd498c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:24df6efc
$ dmesg | grep -i kill
