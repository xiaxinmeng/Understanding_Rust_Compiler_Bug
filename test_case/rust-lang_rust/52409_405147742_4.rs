\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-32377.rs","byte_start":657,"byte_end":671,"line_start":23,"line_end":23,"column_start":14,"column_end":28,"is_primary":true,"text":[{"text":"    unsafe { mem::transmute(x) }","highlight_start":14,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"source type: [usize; 2] (64 bits)","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"target type: Bar<U> (0 bits)","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0512]: transmute called with types of different sizes\n  --> /checkout/src/test/ui/issue-32377.rs:23:14\n   |\nLL |     unsafe { mem::transmute(x) }\n   |              ^^^^^^^^^^^^^^\n   |\n   = note: source type: [usize; 2] (64 bits)\n   = note: target type: Bar<U> (0 bits)\n\n"}
[00:46:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:07] {"message":"For more information about this error, try `rustc --explain E0512`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0512`.\n"}
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] thread '[ui] ui/issue-32377.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:46:07] 
---
[00:46:07] test result: FAILED. 2118 passed; 4 failed; 5 ignored; 0 measured; 0 filtered out
[00:46:07] 
[00:46:07] 
[00:46:07] 
[00:46:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:07] 
[00:46:07] 
[00:46:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:46:07] Build completed unsuccessfully in 0:43:13
---
travis_time:end:0f08514d:start=1531715930711386658,finish=1531715930718688019,duration=7301361
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26eb0b00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:226fd67f
travis_time:start:226fd67f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:017b79aa
$ dmesg | grep -i kill
