\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-22644.rs","byte_start":576,"byte_end":584,"line_start":13,"line_end":13,"column_start":20,"column_end":28,"is_primary":true,"text":[{"text":"    println!(\"{}\", a: usize < 4); //~ ERROR `<` is interpreted as a start of generic","highlight_start":20,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(type_ascription)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: type ascription is experimental (see issue #23416)\n  --> /checkout/src/test/ui/issues/issue-22644.rs:13:20\n   |\nLL |     println!(\"{}\", a: usize < 4); //~ ERROR `<` is interpreted as a start of generic\n   |                    ^^^^^^^^\n   |\n   = help: add #![feature(type_ascription)] to the crate attributes to enable\n\n"}
[00:51:04] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[00:51:04] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:51:04] ------------------------------------------
[00:51:04] 
[00:51:04] thread '[ui] ui/issues/issue-22644.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:51:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:04] test result: FAILED. 5190 passed; 1 failed; 27 ignored; 0 measured; 0 filtered out
[00:51:04] 
[00:51:04] 
[00:51:04] 
[00:51:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:04] 
[00:51:04] 
[00:51:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:51:04] Build completed unsuccessfully in 0:48:55
---
travis_time:end:217eaa52:start=1546289758373373245,finish=1546289758380612578,duration=7239333
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00de7a90
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a6598e0
travis_time:start:0a6598e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13eaf484
$ dmesg | grep -i kill
