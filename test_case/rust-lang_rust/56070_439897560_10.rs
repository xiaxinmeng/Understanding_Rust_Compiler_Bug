\n\nStatics are shared everywhere, and if they refer to mutable data one might\nviolate memory safety since holding multiple mutable references to shared data\nis not allowed.\n\nIf you really want global mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0388.rs","byte_start":728,"byte_end":734,"line_start":17,"line_end":17,"column_start":38,"column_end":44,"is_primary":true,"text":[{"text":"static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017","highlight_start":38,"highlight_end":44}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/error-codes/E0388.rs:17:38\n   |\nLL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017\n   |                                      ^^^^^^ statics require immutable values\n\n"}
[00:54:37] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:54:37] {"message":"Some errors occurred: E0017, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0017, E0596.\n"}
[00:54:37] {"message":"For more information about an error, try `rustc --explain E0017`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0017`.\n"}
[00:54:37] ------------------------------------------
[00:54:37] 
[00:54:37] thread '[ui] ui/error-codes/E0388.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:54:37] 
[00:54:37] 
[00:54:37] ---- [ui] ui/write-to-static-mut-in-static.rs stdout ----
[00:54:37] diff of stderr:
[00:54:37] 
[00:54:37] - error[E0658]: statements in statics are unstable (see issue #48821)
[00:54:37] + error: cannot mutate statics in the initializer of another static
[00:54:37] 3    |
[00:54:37] 3    |
[00:54:37] 4 LL | pub static mut B: () = unsafe { A = 1; };
[00:54:37] 5    |                                 ^^^^^
[00:54:37] -    |
[00:54:37] -    = help: add #![feature(const_let)] to the crate attributes to enable
[00:54:37] 8 
[00:54:37] 8 
[00:54:37] - error[E0658]: statements in statics are unstable (see issue #48821)
[00:54:37] + error: cannot mutate statics in the initializer of another static
[00:54:37] 11    |
[00:54:37] 11    |
[00:54:37] 12 LL | pub static mut C: u32 = unsafe { C = 1; 0 };
[00:54:37] 13    |                                  ^^^^^
[00:54:37] -    |
[00:54:37] -    = help: add #![feature(const_let)] to the crate attributes to enable
[00:54:37] 16 
---
[00:54:37] 20 
[00:54:37] 
[00:54:37] 
[00:54:37] The actual stderr differed from the expected stderr.
[00:54:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/write-to-static-mut-in-static.stderr
[00:54:37] To update references, rerun the tests and pass the `--bless` flag
[00:54:37] To only update this specific test, also pass `--test-args write-to-static-mut-in-static.rs`
[00:54:37] error: 1 errors occurred comparing output.
[00:54:37] status: exit code: 1
[00:54:37] status: exit code: 1
[00:54:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/write-to-static-mut-in-static.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/auxiliary" "-A" "unused"
[00:54:37] ------------------------------------------
[00:54:37] 
[00:54:37] ------------------------------------------
[00:54:37] stderr:
[00:54:37] stderr:
[00:54:37] ------------------------------------------
[00:54:37] {"message":"cannot mutate statics in the initializer of another static","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/write-to-static-mut-in-static.rs","byte_start":550,"byte_end":555,"line_start":14,"line_end":14,"column_start":33,"column_end":38,"is_primary":true,"text":[{"text":"pub static mut B: () = unsafe { A = 1; };","highlight_start":33,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot mutate statics in the initializer of another static\n  --> /checkout/src/test/ui/write-to-static-mut-in-static.rs:14:33\n   |\nLL | pub static mut B: () = unsafe { A = 1; };\n   |                                 ^^^^^\n\n"}
[00:54:37] {"message":"cannot mutate statics in the initializer of another static","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/write-to-static-mut-in-static.rs","byte_start":640,"byte_end":645,"line_start":17,"line_end":17,"column_start":34,"column_end":39,"is_primary":true,"text":[{"text":"pub static mut C: u32 = unsafe { C = 1; 0 };","highlight_start":34,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot mutate statics in the initializer of another static\n  --> /checkout/src/test/ui/write-to-static-mut-in-static.rs:17:34\n   |\nLL | pub static mut C: u32 = unsafe { C = 1; 0 };\n   |                                  ^^^^^\n\n"}
[00:54:37] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:54:37] ------------------------------------------
[00:54:37] 
[00:54:37] thread '[ui] ui/write-to-static-mut-in-static.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:54:37] 
---
[00:54:37] 
[00:54:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:54:37] 
[00:54:37] 
[00:54:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:37] 
[00:54:37] 
[00:54:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:37] Build completed unsuccessfully in 0:03:52
[00:54:37] Build completed unsuccessfully in 0:03:52
[00:54:37] make: *** [check] Error 1
[00:54:37] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03838076
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 19 13:45:49 UTC 2018
---
travis_time:end:0ef77106:start=1542635151099701050,finish=1542635151105064874,duration=5363824
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19a46870
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01d63bd6
travis_time:start:01d63bd6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bc72349
$ dmesg | grep -i kill
