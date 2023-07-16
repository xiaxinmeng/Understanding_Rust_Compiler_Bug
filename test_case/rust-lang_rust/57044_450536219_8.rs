\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/transmute/main.rs","byte_start":800,"byte_end":809,"line_start":29,"line_end":29,"column_start":18,"column_end":27,"is_primary":true,"text":[{"text":"    let x: Foo = transmute(10); //~ ERROR cannot transmute between types of different sizes","highlight_start":18,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"source type: `i32` (32 bits)","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"target type: `Foo` (0 bits)","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0512]: cannot transmute between types of different sizes, or dependently-sized types\n  --> /checkout/src/test/ui/transmute/main.rs:29:18\n   |\nLL |     let x: Foo = transmute(10); //~ ERROR cannot transmute between types of different sizes\n   |                  ^^^^^^^^^\n   |\n   = note: source type: `i32` (32 bits)\n   = note: target type: `Foo` (0 bits)\n\n"}
[00:52:07] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:52:07] {"message":"For more information about this error, try `rustc --explain E0512`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0512`.\n"}
[00:52:07] ------------------------------------------
[00:52:07] 
[00:52:07] thread '[ui] ui/transmute/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:52:07] 
---
[00:52:07] 
[00:52:07] 
[00:52:07] 
[00:52:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:52:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:07] 
[00:52:07] 
[00:52:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:52:07] Build completed unsuccessfully in 0:49:44
---
travis_time:end:0131190f:start=1546139507925159145,finish=1546139507935180849,duration=10021704
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0101f778
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c9ea840
travis_time:start:1c9ea840
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:034c2b41
$ dmesg | grep -i kill
