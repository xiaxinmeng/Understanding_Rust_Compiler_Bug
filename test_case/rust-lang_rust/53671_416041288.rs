plain
[00:45:52] ....................................................................................................
[00:45:55] ....................................................................................................
[00:45:58] ....................................................................................................
[00:46:02] ........i...........................................................................................
[00:46:06] ..........F.......................................................................................i.
[00:46:13] ............ii.iii..................................................................................
[00:46:15] ....................................................................................................
[00:46:17] ....................................................................................................
[00:46:20] ....................................................................................................
---
[00:47:51] 1 error: this expression will panic at runtime
[00:47:51] -   --> $DIR/const-err2.rs:25:13
[00:47:51] +   --> $DIR/const-err2.rs:27:13
[00:47:51] 3    |
[00:47:51] - LL |     let a = -std::i8::MIN;
[00:47:51] -    |             ^^^^^^^^^^^^^ attempt to negate with overflow
[00:47:51] + LL |     let b = 200u8 + 200u8 + 200u8;
[00:47:51] +    |             ^^^^^^^^^^^^^ attempt to add with overflow
[00:47:51] 7 note: lint level defined here
[00:47:51] 8   --> $DIR/const-err2.rs:18:9
[00:47:51] 
[00:47:51] 11    |         ^^^^^^^^^
[00:47:51] 11    |         ^^^^^^^^^
[00:47:51] 12 
[00:47:51] 13 error: this expression will panic at runtime
[00:47:51] -   --> $DIR/const-err2.rs:27:13
[00:47:51] -    |
[00:47:51] - LL |     let b = 200u8 + 200u8 + 200u8;
[00:47:51] -    |             ^^^^^^^^^^^^^ attempt to add with overflow
[00:47:51] - error: this expression will panic at runtime
[00:47:51] 20   --> $DIR/const-err2.rs:29:13
[00:47:51] 21    |
[00:47:51] 21    |
[00:47:51] 22 LL |     let c = 200u8 * 4;
[00:47:51] 
[00:47:51] 34 LL |     let _e = [5u8][1];
[00:47:51] 36 
[00:47:51] - error: aborting due to 5 previous errors
[00:47:51] + error: aborting due to 4 previous errors
[00:47:51] 38 
[00:47:51] 38 
[00:47:51] 39 
[00:47:51] 
[00:47:51] 
[00:47:51] The actual stderr differed from the expected stderr.
[00:47:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2/const-err2.stderr
[00:47:51] To update references, rerun the tests and pass the `--bless` flag
[00:47:51] To only update this specific test, also pass `--test-args consts/const-err2.rs`
[00:47:51] error: 1 errors occurred comparing output.
[00:47:51] status: exit code: 1
[00:47:51] status: exit code: 1
[00:47:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err2/auxiliary" "-A" "unused"
[00:47:51] ------------------------------------------
[00:47:51] 
[00:47:51] ------------------------------------------
[00:47:51] stderr:
[00:47:51] stderr:
[00:47:51] ------------------------------------------
[00:47:51] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err2.rs","byte_start":867,"byte_end":880,"line_start":27,"line_end":27,"column_start":13,"column_end":26,"is_primary":true,"text":[{"text":"    let b = 200u8 + 200u8 + 200u8;","highlight_start":13,"highlight_end":26}],"label":"attempt to add with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err2.rs","byte_start":730,"byte_end":739,"line_start":18,"line_end":18,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![deny(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-err2.rs:27:13\n   |\nLL |     let b = 200u8 + 200u8 + 200u8;\n   |             ^^^^^^^^^^^^^ attempt to add with overflow\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-err2.rs:18:9\n   |\nLL | #![deny(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:47:51] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err2.rs","byte_start":927,"byte_end":936,"line_start":29,"line_end":29,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let c = 200u8 * 4;","highlight_start":13,"highlight_end":22}],"label":"attempt to multiply with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-err2.rs:29:13\n   |\nLL |     let c = 200u8 * 4;\n   |             ^^^^^^^^^ attempt to multiply with overflow\n\n"}
[00:47:51] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err2.rs","byte_start":975,"byte_end":992,"line_start":31,"line_end":31,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"    let d = 42u8 - (42u8 + 1);","highlight_start":13,"highlight_end":30}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this expression will panic at runtime\n  --> /checkout/src/test/ui/consts/const-err2.rs:31:13\n   |\nLL |     let d = 42u8 - (42u8 + 1);\n   |             ^^^^^^^^^^^^^^^^^ attempt to subtract with overflow\n\n"}
[00:47:51] {"message":"index out of bounds: the len is 1 but the index is 1","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err2.rs","byte_start":1032,"byte_end":1040,"line_start":33,"line_end":33,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    let _e = [5u8][1];","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: index out of bounds: the len is 1 but the index is 1\n  --> /checkout/src/test/ui/consts/const-err2.rs:33:14\n   |\nLL |     let _e = [5u8][1];\n   |              ^^^^^^^^\n\n"}
[00:47:51] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:47:51] ------------------------------------------
[00:47:51] 
[00:47:51] thread '[ui] ui/consts/const-err2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:51] 
[00:47:51] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:47:51] 
[00:47:51] 
[00:47:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:51] 
[00:47:51] 
[00:47:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:51] Build completed unsuccessfully in 0:03:09
[00:47:51] Build completed unsuccessfully in 0:03:09
[00:47:51] Makefile:58: recipe for target 'check' failed
[00:47:51] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0018d024
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e1f39ba:start=1535292302781650172,finish=1535292302790262053,duration=8611881
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11f4479e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cad7ae4
travis_time:start:0cad7ae4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01471326
$ dmesg | grep -i kill
