\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":1333,"byte_end":1344,"line_start":52,"line_end":52,"column_start":12,"column_end":23,"is_primary":true,"text":[{"text":"    pub d: PhantomData<u8>,","highlight_start":12,"highlight_end":23}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"possible candidate is found in another module, you can import it into scope","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":153,"byte_end":153,"line_start":6,"line_end":6,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"/// Zero-length arrays themselves \"have VLA\".","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::marker::PhantomData;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0412]: cannot find type `PhantomData` in this scope\n  --> /checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs:52:12\n   |\nLL |     pub d: PhantomData<u8>,\n   |            ^^^^^^^^^^^ not found in this scope\nhelp: possible candidate is found in another module, you can import it into scope\n   |\nLL | use std::marker::PhantomData;\n   |\n\n"}
[01:07:34] {"message":"For more information about this error, try `rustc --explain E0412`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0412`.\n"}
[01:07:34] 
[01:07:34] ------------------------------------------
[01:07:34] 
[01:07:34] 
[01:07:34] thread '[ui] ui/layout/has-vla-zero-length-array-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:07:34] 
[01:07:34] 
[01:07:34] failures:
[01:07:34] failures:
[01:07:34]     [ui] ui/layout/has-vla-zero-length-array-struct.rs
[01:07:34] test result: FAILED. 5284 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
[01:07:34] 
[01:07:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:07:34] 
[01:07:34] 
[01:07:34] 
[01:07:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:34] 
[01:07:34] 
[01:07:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:34] Build completed unsuccessfully in 0:04:18
[01:07:34] Build completed unsuccessfully in 0:04:18
[01:07:34] make: *** [check] Error 1
[01:07:34] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f4ef466
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 16 00:23:08 UTC 2019
---
travis_time:end:0bfda4da:start=1547598189573863941,finish=1547598189578668564,duration=4804623
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:137245da
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:38864448
travis_time:start:38864448
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04044259
$ dmesg | grep -i kill
