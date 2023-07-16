\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-infer-borrow-scope-too-big.rs","byte_start":721,"byte_end":723,"line_start":23,"line_end":23,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    return xc;","highlight_start":12,"highlight_end":14}],"label":"returns a value referencing data owned by the current function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/regions/regions-infer-borrow-scope-too-big.rs","byte_start":639,"byte_end":642,"line_start":21,"line_end":21,"column_start":22,"column_end":25,"is_primary":false,"text":[{"text":"    let xc = x_coord(&*p); //~ ERROR `*p` does not live long enough","highlight_start":22,"highlight_end":25}],"label":"`*p` is borrowed here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0515]: cannot return value referencing local data `*p`\n  --> /checkout/src/test/ui/regions/regions-infer-borrow-scope-too-big.rs:23:12\n   |\nLL |     let xc = x_coord(&*p); //~ ERROR `*p` does not live long enough\n   |                      --- `*p` is borrowed here\nLL |     assert_eq!(*xc, 3);\nLL |     return xc;\n   |            ^^ returns a value referencing data owned by the current function\n\n"}
[01:20:28] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:28] {"message":"For more information about this error, try `rustc --explain E0515`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0515`.\n"}
[01:20:28] ------------------------------------------
[01:20:28] 
[01:20:28] thread '[ui (nll)] ui/regions/regions-infer-borrow-scope-too-big.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:28] 
---
[01:20:28] test result: FAILED. 5100 passed; 11 failed; 88 ignored; 0 measured; 0 filtered out
[01:20:28] 
[01:20:28] 
[01:20:28] 
[01:20:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:20:28] 
[01:20:28] 
[01:20:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:28] Build completed unsuccessfully in 0:07:10
[01:20:28] Build completed unsuccessfully in 0:07:10
[01:20:28] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:016603d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 24 11:59:27 UTC 2018
---
travis_time:end:09f4e63c:start=1545652769185968508,finish=1545652769193274223,duration=7305715
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d77a434
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:088ec352
travis_time:start:088ec352
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f74c90
$ dmesg | grep -i kill
