plain
[00:42:11] ....................................................................................................
[00:42:14] ....................................................................................................
[00:42:17] ....................................................................................................
[00:42:20] ...................................................................i................................
[00:42:23] .................................F......................................i...........................
[00:42:30] ....................................................................................................
[00:42:33] ...........................................................................................i........
[00:42:36] ..............................................................................i....
[00:42:36] failures:
[00:42:36] failures:
[00:42:36] 
[00:42:36] ---- [ui] ui/nll/issue-52133.rs#compare stdout ----
[00:42:36] diff of stderr:
[00:42:36] 
[00:42:36] - error[E0597]: `*value` does not live long enough (Ast)
[00:42:36] -    |
[00:42:36] -    |
[00:42:36] - LL |         data.push(value); //[compare]~ ERROR `*value` does not live long enough (Ast)
[00:42:36] -    |                   ^^^^^ borrowed value does not live long enough
[00:42:36] -    |
[00:42:36] - note: borrowed value must be valid for the lifetime 'b as defined on the function body at 45:20...
[00:42:36] -    |
[00:42:36] -    |
[00:42:36] - LL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {
[00:42:36] -    |                    ^^
[00:42:36] - note: ...but borrowed value is only valid for the lifetime 'a as defined on the function body at 45:16
[00:42:36] -    |
[00:42:36] -    |
[00:42:36] - LL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {
[00:42:36] - 
[00:42:36] 18 error[E0623]: lifetime mismatch
[00:42:36] 19   --> $DIR/issue-52133.rs:46:9
[00:42:36] 20    |
[00:42:36] 20    |
[00:42:36] 
[00:42:36] 23 LL |     let x = move || { //[compare]~ ERROR lifetime mismatch
[00:42:36] 24    |         ^ ...but data from `value` flows into `data` here
[00:42:36] - error: aborting due to 2 previous errors
[00:42:36] + error: aborting due to previous error
[00:42:36] 27 
[00:42:36] - Some errors occurred: E0597, E0623.
[00:42:36] - Some errors occurred: E0597, E0623.
[00:42:36] - For more information about an error, try `rustc --explain E0597`.
[00:42:36] + For more information about this error, try `rustc --explain E0623`.
[00:42:36] 30 
[00:42:36] 
[00:42:36] 
[00:42:36] The actual stderr differed from the expected stderr.
[00:42:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52133.compare/issue-52133.compare.stderr
[00:42:36] To update references, rerun the tests and pass the `--bless` flag
[00:42:36] To only update this specific test, also pass `--test-args nll/issue-52133.rs`
[00:42:36] 
[00:42:36] error in revision `compare`: 1 errors occurred comparing output.
[00:42:36] status: exit code: 1
[00:42:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52133.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "compare" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52133.compare/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52133.compare/auxiliary" "-A" "unused"
[00:42:36] ------------------------------------------
[00:42:36] 
[00:42:36] ------------------------------------------
[00:42:36] stderr:
[00:42:36] stderr:
[00:42:36] ------------------------------------------
[00:42:36] {"message":"lifetime mismatch","code":{"code":"E0623","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-52133.rs","byte_start":1231,"byte_end":1238,"line_start":45,"line_end":45,"column_start":63,"column_end":70,"is_primary":false,"text":[{"text":"fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {","highlight_start":63,"highlight_end":70}],"label":"these two types are declared with different lifetimes...","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-52133.rs","byte_start":1202,"byte_end":1222,"line_start":45,"line_end":45,"column_start":34,"column_end":54,"is_primary":false,"text":[{"text":"fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {","highlight_start":34,"highlight_end":54}],"label":"","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-52133.rs","byte_start":1271,"byte_end":1272,"line_start":46,"line_end":46,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x = move || { //[compare]~ ERROR lifetime mismatch","highlight_start":9,"highlight_end":10}],"label":"...but data from `value` flows into `data` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0623]: lifetime mismatch\n  --> /checkout/src/test/ui/nll/issue-52133.rs:46:9\n   |\nLL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {\n   |                                  --------------------         ------- these two types are declared with different lifetimes...\nLL |     let x = move || { //[compare]~ ERROR lifetime mismatch\n   |         ^ ...but data from `value` flows into `data` here\n\n"}
[00:42:36] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:36] {"message":"For more information about this error, try `rustc --explain E0623`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0623`.\n"}
[00:42:36] ------------------------------------------
[00:42:36] 
[00:42:36] thread '[ui] ui/nll/issue-52133.rs#compare' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:42:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:42:36] 
[00:42:36] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:42:36] 
[00:42:36] 
[00:42:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:36] 
[00:42:36] 
[00:42:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:36] Build completed unsuccessfully in 0:01:36
[00:42:36] Build completed unsuccessfully in 0:01:36
[00:42:36] Makefile:58: recipe for target 'check' failed
[00:42:36] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fb917b6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:15775403:start=1532709762345940994,finish=1532709762354889916,duration=8948922
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03da3761
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:172e4f39
travis_time:start:172e4f39
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14bde200
$ dmesg | grep -i kill
